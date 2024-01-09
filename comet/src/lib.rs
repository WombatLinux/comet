use std::collections::HashMap;
use std::env::set_current_dir;
use std::ffi::c_char;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tar::Archive;
use crate::config::Config;
use crate::package::{Package, SemVer};
use crate::repo::Repository;
use sha2::{Sha256, Digest};

pub mod package;
pub mod repo;
mod config;

/// Installs a package from a repository or a local file
///
/// Where applicable, also installs dependencies
///
/// # Arguments
/// * `package` - The name of the package to install
/// * `local` - Whether or not to install a local package
/// * `force` - Whether or not to force install a package
pub fn install_package(package: String, local: bool, force: bool) -> Result<(), String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            return Err(format!("Error while reading config file: {}", err));
        }
    };

    let repo_file = format!("{}/repo.yml", config.storage_dir);
    let installed = Repository::from_file(repo_file);

    // if the package is already installed, then exit if force is false
    if installed.get_package(package.clone()).is_some() && !force {
        return Err(format!("Package {} is already installed. Use --force to reinstall", package));
    }

    let file_path;
    let package_name;

    if !local {
        package_name = package.clone();

        if download_package(package.clone()).is_err() {
            return Err(format!("Package {} not found", package));
        }

        file_path = format!("{}/{}.star", config.tmp_dir, package);
    } else {
        // if package isn't in the temp directory, then copy it to the temp directory
        // the package file would be the "package" argument
        let package_path = Path::new(&package);

        if !package_path.exists() {
            return Err(format!("Package {} not found", package));
        }

        let file_name = package_path.file_name().unwrap().to_str().unwrap();
        file_path = format!("{}/{}", config.tmp_dir, file_name);

        if !Path::new(&file_path).exists() {
            std::fs::copy(package_path, file_path.clone()).unwrap();
        }

        // package name is filename without extension
        package_name = file_name.split(".").collect::<Vec<&str>>()[0].to_string();
    }

    // lets move to the temp directory
    if set_current_dir(config.tmp_dir.clone()).is_err() {
        return Err(format!("Error while moving to temp directory: {}", config.tmp_dir));
    }

    // extract the package
    let file = match File::open(file_path.clone()) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Error while opening package file: {}", err));
        }
    };

    let mut archive = Archive::new(file);

    // create the package directory
    let package_dir = format!("{}/{}", config.tmp_dir, package_name);

    if !Path::new(&package_dir).exists() {
        std::fs::create_dir(&package_dir).unwrap();
    }

    archive.unpack(&package_dir).unwrap();

    // move to the package directory
    set_current_dir(package_dir.clone()).unwrap();

    // read the package file
    let package_file = format!("{}/info.yml", package_dir);

    let package = Package::from_file(package_file.clone());

    // check dependencies
    // dependencies are a hashmap of package name to minimum version
    for (key, value) in package.dependencies.clone() {
        if check_dependency(key.clone(), value.clone()).is_err() {
            return Err(format!("Dependency {} not found", key));
        }
    }

    // since check dependency installs the package if it can, we can just install the package
    // if it isn't installed

    // to install, we run the "install" script

    // first check if the install script exists
    let install_script = format!("{}/install", package_dir);

    if Path::new(&install_script).exists() {
        // if it does, run it
        let command = format!("sh {}", install_script);
        std::process::Command::new(command).spawn().unwrap();
    }

    // move the uninstall script to the storage/scripts directory
    let uninstall_script = format!("{}/remove", package_dir);

    // create the scripts directory if it doesn't exist
    if !Path::new(&format!("{}/scripts", config.storage_dir)).exists() {
        std::fs::create_dir(format!("{}/scripts", config.storage_dir)).unwrap();
    }

    if Path::new(&uninstall_script).exists() {
        let script_name = format!("{}/scripts/{}", config.storage_dir, package_name);
        std::fs::copy(uninstall_script, script_name).unwrap();
    }

    // move back to the temp directory
    set_current_dir(config.tmp_dir.clone()).unwrap();

    // remove the package directory
    std::fs::remove_dir_all(package_dir.clone()).unwrap();

    // remove the package file
    std::fs::remove_file(file_path.clone()).unwrap();

    // add the package to the repo
    let repo_file = format!("{}/repo.yml", config.storage_dir);
    let mut repo = Repository::from_file(repo_file);

    repo.add_package(package);

    repo.to_file(format!("{}/repo.yml", config.storage_dir));

    // if keep_package_files is false, remove the package file from the temp directory
    if !config.keep_package_files {
        std::fs::remove_file(file_path.clone()).unwrap();
    }

    Ok(())
}

/// Uninstalls a package
///
/// # Arguments
/// * `package` - The name of the package to uninstall
/// * `force` - Whether or not to force uninstall a package
pub fn remove_package(package: String, force: bool) -> Result<(), String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            return Err(format!("Error while reading config file: {}", err));
        }
    };

    let repo_file = format!("{}/repo.yml", config.storage_dir);
    let mut repo = Repository::from_file(repo_file);

    let package_file = repo.get_package(package.clone());

    if package_file.is_none() {
        return Err(format!("Package {} not found", package));
    }

    if repo.is_dependency(package.clone()) && !force {
        return Err(format!("Package {} is a dependency of another package. Use --force to remove", package));
    }

    // remove the package

    // go to the storage/scripts directory
    set_current_dir(format!("{}/scripts", config.storage_dir)).unwrap();

    // run the remove script
    let script_name = format!("{}/scripts/{}", config.storage_dir, package);

    if Path::new(&script_name).exists() {
        std::process::Command::new(script_name).spawn().unwrap();
    }

    // remove the package from the repo
    repo.remove_package(package.clone());

    Ok(())
}

/// Updates a package
///
/// Basically just installs it again but with the --force flag
///
/// # Arguments
/// * `package` - The name of the package to update
pub fn update_package(package: String) -> Result<(), String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            return Err(format!("Error while reading config file: {}", err));
        }
    };

    let repo_file = format!("{}/repo.yml", config.storage_dir);
    let repo = Repository::from_file(repo_file);

    let package_file = repo.get_package(package.clone());

    if package_file.is_none() {
        return Err(format!("Package {} not found. To install, use the install command", package));
    }

    // check if an update is available using the cache
    let cache_file = format!("{}/cache.yml", config.storage_dir);
    let cache = Repository::from_file(cache_file);
    let cache_package = cache.get_package(package.clone());

    if cache_package.is_none() {
        return Err(format!("Package {} not found. Update the cache and try again", package));
    }

    let cache_package = cache_package.unwrap();
    let cache_version = SemVer::from_string(cache_package.version.clone());
    let installed_package = repo.get_package(package.clone()).unwrap();

    let installed_version = SemVer::from_string(installed_package.version.clone());

    if cache_version <= installed_version {
        return Err(format!("Package {} is already up to date", package));
    }

    // if the package is found, update it by installing it again
    if install_package(package.clone(), false, true).is_err() {
        return Err(format!("Error while updating package {}", package));
    }

    Ok(())
}


/// Downloads a package from a repository and stores it in the temp directory
///
/// # Arguments
/// * `package` - The name of the package to download
pub fn download_package(package: String) -> Result<(), String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            return Err(format!("Error while reading config file: {}", err));
        }
    };

    if !package_is_cached(package.clone()) {
        return Err(format!("Package {} not found. Update the cache and try again", package));
    }

    // TODO: Determine if we should just attempt to download the package from the repositories

    // package is found, lets download it from an available repository
    let repository = find_package_file_from_repo(package.clone());

    if repository.is_none() {
        return Err(format!("Package {} not found. Update the cache and try again", package));
    }

    let repository = repository.unwrap();
    let url = format!("{}/{}.star", repository, package);

    let mut response = reqwest::blocking::get(&url).unwrap();

    let mut file = File::create(format!("{}/{}.star", config.tmp_dir, package)).unwrap();
    response.copy_to(&mut file).unwrap();

    // load the cache
    let cache_file = format!("{}/cache.yml", config.storage_dir);
    let mut cache = Repository::from_file(cache_file);

    // grab the package from the cache
    let package_file = cache.get_package(package.clone()).unwrap();

    // check the checksum
    if package_file.checksum.is_some() {
        let checksum = package_file.checksum.clone().unwrap();
        let mut file = File::open(format!("{}/{}.star", config.tmp_dir, package)).unwrap();

        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return Err(format!("Error while reading package file"));
        }

        let mut hasher = Sha256::new();
        hasher.update(buffer);

        let hash = format!("{:x}", hasher.finalize());
        if hash != checksum {
            return Err(format!("Checksum mismatch for package {}", package));
        }
    }

    Ok(())
}

/// Finds a package file from a repository
///
/// # Arguments
/// * `package` - The name of the package to find
///
/// # Returns
/// * `Option<String>` - The URL of the repository that contains the package
/// * `Option<String>` - None if the package can't be found
fn find_package_file_from_repo(package: String) -> Option<String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("Error while reading config file: {}", err);
        }
    };

    for repository in config.repositories {
        let repo = Repository::from_web(repository.clone());

        let package_file = repo.get_package(package.clone());

        if package_file.is_some() {
            return Some(repository.clone());
        }
    }

    None
}

/// Updates the local cache of packages by downloading the repository files from the repositories
pub fn update_cache_file() -> Result<(), String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            return Err(format!("Error while reading config file: {}", err));
        }
    };

    let cache_file = format!("{}/cache.yml", config.storage_dir);

    // remove the old cache file
    if Path::new(&cache_file).exists() {
        std::fs::remove_file(cache_file.clone()).unwrap();
    }

    let cache = Repository::new(true);

    for repository in config.repositories {
        let mut repo = Repository::from_string(repository.clone());
        let repo_file = Repository::from_web(repository.clone());

        for (name, value) in repo_file.packages {
            if !repo.packages.contains_key(&name) {
                repo.add_package(value);
            } else {
                // see if the version is greater than the one in the repo
                let repo_package = repo.get_package(name.clone()).unwrap();
                let repo_version = SemVer::from_string(repo_package.version.clone());
                let value_version = SemVer::from_string(value.version.clone());

                if value_version > repo_version {
                    repo.add_package(value);
                }
            }
        }
    }

    cache.to_file(format!("{}/cache.yml", config.storage_dir));

    Ok(())
}

/// Checks if a package is cached by looking in the cache file
///
/// If it is, it returns true
///
/// If it isn't, it returns false
///
/// # Arguments
/// * `package` - The name of the package to check
///
/// # Returns
/// * `bool` - Whether or not the package is cached
fn package_is_cached(package: String) -> bool {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("Error while reading config file: {}", err);
        }
    };

    let cache_file = format!("{}/cache.yml", config.storage_dir);

    let cache = Repository::from_file(cache_file);

    let package_file = cache.get_package(package);

    if package_file.is_none() {
        return false;
    }

    true
}

/// Uses package_is_cached to find out if a dependency with the minimum version can be found
/// in any of the repositories
///
/// If it can't, it returns an error
///
/// # Arguments
/// * `dependency` - The name of the dependency to check
/// * `minimum_version` - The minimum version of the dependency to check
///
/// # Returns
/// * `Result<(), String>` - An error if the dependency can't be found
/// * `Result<(), String>` - Ok if the dependency can be found
fn check_dependency(dependency: String, minimum_version: String) -> Result<(), String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("Error while reading config file: {}", err);
        }
    };

    let mut found = false;

    // first see if it is installed
    let repo_file = format!("{}/repo.yml", config.storage_dir);
    let repo = Repository::from_file(repo_file);
    let package = repo.get_package(dependency.clone());
    if package.is_some() {
        let package = package.unwrap();
        let package_version = SemVer::from_string(package.version.clone());
        let minimum_version = SemVer::from_string(minimum_version.clone());

        if package_version >= minimum_version {
            return Ok(());
        }
    }

    // if it isn't installed, see if it is cached and if it is, check the version
    // if the version is greater than or equal to the minimum version, install it then return
    // ok
    if package_is_cached(dependency.clone()) {
        let cache_file = format!("{}/cache.yml", config.storage_dir);
        let cache = Repository::from_file(cache_file);
        let package = cache.get_package(dependency.clone()).unwrap();
        let package_version = SemVer::from_string(package.version.clone());
        let minimum_version = SemVer::from_string(minimum_version.clone());

        if package_version >= minimum_version {
            found = true;
        }
    }

    // if it isn't installed or cached, then as far as we know it doesn't exist so return an error
    if !found {
        return Err(format!("Dependency {} not found", dependency));
    }

    // if its cached but not installed, install it (or at least try to)
    if install_package(dependency.clone(), false, false).is_err() {
        return Err(format!("Dependency {} could not be installed", dependency));
    }

    Ok(())
}

/// Sets up the system by creating the config file and storage directory
pub fn setup() -> Result<(), String>{
    // first, create the containing directory
    let config_location;

    // if linux
    #[cfg(target_os = "linux")]
    {
        config_location = "/etc/comet".to_string();
    }

    // if windows
    #[cfg(target_os = "windows")]
    {
        config_location = "C:\\Program Files\\Comet".to_string();
    }

    // if mac
    #[cfg(target_os = "macos")]
    {
        config_location = "/Library/Application Support/Comet".to_string();
    }

    // if the directory doesn't exist, create it
    if !Path::new(&config_location).exists() {
        if std::fs::create_dir_all(config_location.clone()).is_err() {
            return Err(format!("Error while creating config directory: {}", config_location));
        }
    }

    // now create the config file
    let config_location;

    // if linux
    #[cfg(target_os = "linux")]
    {
        config_location = "/etc/comet/config.yml".to_string();
    }

    // if windows
    #[cfg(target_os = "windows")]
    {
        config_location = "C:\\Program Files\\Comet\\config.yml".to_string();
    }

    // if mac
    #[cfg(target_os = "macos")]
    {
        config_location = "/Library/Application Support/Comet/config.yml".to_string();
    }

    let mut config = Config::new(vec![], false, "".to_string(), "".to_string());

    // if the config file doesn't exist, create it
    if !Path::new(&config_location).exists() {
        if std::fs::create_dir_all(config_location.clone()).is_err() {
            return Err(format!("Error while creating config file: {}", config_location));
        }

        // if linux
        #[cfg(target_os = "linux")]
        {
            config = Config::new(vec!["https://repo.wombatlinux.org".to_string()], false, "/var/lib/comet".to_string(), "/tmp".to_string());
        }

        // if windows
        #[cfg(target_os = "windows")]
        {
            config = Config::new(vec!["https://repo.wombatlinux.org".to_string()], false, "C:\\Program Files\\Comet".to_string(), "C:\\Windows\\Temp".to_string());
        }

        // if mac
        #[cfg(target_os = "macos")]
        {
            config = Config::new(vec!["https://repo.wombatlinux.org".to_string()], false, "/Library/Application Support/Comet".to_string(), "/tmp".to_string());
        }

        // write the config to the file

        if std::fs::write(config_location.clone(), config.to_string()).is_err() {
            return Err(format!("Error while writing config file: {}", config_location));
        }
    }

    // create the storage directory and tmp directory if they don't exist
    // use the config file to get the storage directory and tmp directory

    if !Path::new(&config.tmp_dir.clone()).exists() {
        if std::fs::create_dir_all(config.tmp_dir.clone()).is_err() {
            return Err(format!("Error while creating temp directory: {}", config.tmp_dir));
        }
    }

    if !Path::new(&config.storage_dir.clone()).exists() {
        if std::fs::create_dir_all(config.storage_dir.clone()) .is_err() {
            return Err(format!("Error while creating storage directory: {}", config.storage_dir));
        }
    }

    // only create the cache file if it doesn't exist
    let cache_file = format!("{}/cache.yml", config.storage_dir.clone());
    if !Path::new(&cache_file).exists() {
        let cache = Repository::new(true);
        cache.to_file(cache_file.clone());
    }

    // same with the repo file
    let repo_file = format!("{}/repo.yml", config.storage_dir.clone());
    if !Path::new(&repo_file).exists() {
        let repo = Repository::new(false);
        repo.to_file(repo_file.clone());
    }

    Ok(())
}


/// Lists all installed packages
///
/// # Returns
/// * `HashMap<String, String>` - A hashmap of package name to version
pub fn list_packages() -> HashMap<String, String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("Error while reading config file: {}", err);
        }
    };

    let repo_file = format!("{}/repo.yml", config.storage_dir);
    let repo = Repository::from_file(repo_file);

    let mut packages = HashMap::new();

    for (_, package) in repo.packages {
        packages.insert(package.name, package.version);
    }

    packages
}

/// Lists all available packages
///
/// # Returns
/// * `HashMap<String, String>` - A hashmap of package name to version
pub fn list_available_packages() -> HashMap<String, String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("Error while reading config file: {}", err);
        }
    };

    let cache_file = format!("{}/cache.yml", config.storage_dir);
    let cache = Repository::from_file(cache_file);

    let mut packages = HashMap::new();

    for (_, package) in cache.packages {
        packages.insert(package.name, package.version);
    }

    packages
}


/// Updates all packages
///
/// Basically just runs update_package on all installed packages
pub fn update_all_packages()  {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("Error while reading config file: {}", err);
        }
    };

    let repo_file = format!("{}/repo.yml", config.storage_dir);
    let repo = Repository::from_file(repo_file);

    for (_, package) in repo.packages {
        let _ = update_package(package.name.clone());

        // we don't care if it fails, we just want to try to update all packages
    }
}

pub fn check_permissions() -> bool {
    // make sure the correct directories can be accessed with write permissions
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("Error while reading config file: {}", err);
        }
    };

    let mut can_write = true;

    let locations = vec![config.storage_dir.clone(), config.tmp_dir.clone()];

    for location in locations {
        // check if readonly
        let metadata = std::fs::metadata(location.clone()).unwrap();
        let readonly = metadata.permissions().readonly();

        if readonly {
            can_write = false;
        }
    }

    can_write
}

pub fn get_package_details(package: String) -> Option<String> {
    let config = match Config::from_file() {
        Ok(config) => config,
        Err(err) => {
            panic!("Error while reading config file: {}", err);
        }
    };

    let repo_file = format!("{}/cache.yml", config.storage_dir);
    let repo = Repository::from_file(repo_file);

    let package_file = repo.get_package(package);

    if package_file.is_none() {
        return None;
    }

    let package_file = package_file.unwrap();

    // lets format the package details
    let details = format!("{} - {}\n\n{}\n{}", package_file.name, package_file.version,
                          package_file.description, package_file.license);

    Some(details)
}

/**
BEGIN C COMPATIBLE FUNCTIONS FOR USE IN OTHER LANGUAGES (C, C++, Python, etc.)

These functions are used to make comet usable in other languages, such as if you want to use
comet in a ncurses application written in C or C++ or if you want to use comet in a Python script

If you are using comet in a Rust application, you should use the functions above instead of these
functions, as they are more idiomatic Rust code and are easier to use in Rust applications than
these functions are (since they are written in Rust, for Rust).
**/

#[no_mangle]
pub extern "C" fn install(package: *const c_char, local: bool, force: bool) -> bool {
    let package = unsafe {
        assert!(!package.is_null());

        std::ffi::CStr::from_ptr(package)
    };

    let package = package.to_str().unwrap();

    let result = install_package(package.to_string(), local, force);

    if result.is_err() {
        return false;
    }

    true
}

#[no_mangle]
pub extern "C" fn remove(package: *const c_char, force: bool) -> bool {
    let package = unsafe {
        assert!(!package.is_null());

        std::ffi::CStr::from_ptr(package)
    };

    let package = package.to_str().unwrap();

    let result = remove_package(package.to_string(), force);

    if result.is_err() {
        return false;
    }

    true
}

#[no_mangle]
pub extern "C" fn update(package: *const c_char) -> bool {
    let package = unsafe {
        assert!(!package.is_null());

        std::ffi::CStr::from_ptr(package)
    };

    let package = package.to_str().unwrap();

    let result = update_package(package.to_string());

    if result.is_err() {
        return false;
    }

    true
}

#[no_mangle]
pub extern "C" fn update_all() {
    update_all_packages();
}

#[no_mangle]
pub extern "C" fn list() -> *const c_char {
    let packages = list_packages();

    let mut string = String::new();

    for (key, value) in packages {
        string.push_str(&format!("{}: {}\n", key, value));
    }

    let c_string = std::ffi::CString::new(string).unwrap();

    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn list_available() -> *const c_char {
    let packages = list_available_packages();

    let mut string = String::new();

    for (key, value) in packages {
        string.push_str(&format!("{}: {}\n", key, value));
    }

    let c_string = std::ffi::CString::new(string).unwrap();

    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn update_cache() -> bool {
    let result = update_cache_file();

    if result.is_err() {
        return false;
    }

    true
}

#[no_mangle]
pub extern "C" fn check_perms() -> bool {
    check_permissions()
}

#[no_mangle]
pub extern "C" fn setup_comet() -> bool {
    let result = setup();

    if result.is_err() {
        return false;
    }

    true
}

#[no_mangle]
pub extern "C" fn package_details(package: *const c_char) -> *const c_char {
    let package = unsafe {
        assert!(!package.is_null());

        std::ffi::CStr::from_ptr(package)
    };

    let package = package.to_str().unwrap();

    let details = get_package_details(package.to_string());

    if details.is_none() {
        return std::ptr::null();
    }

    let details = details.unwrap();

    let c_string = std::ffi::CString::new(details).unwrap();

    c_string.into_raw()
}

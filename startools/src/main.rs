use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use clap::{Parser, Subcommand};
use comet;
use comet::package::Package;
use sha2::{Sha256, Digest};

#[derive(Parser)]
#[command(author = "afroraydude", version = "1.0.0", about = "The simple package manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    #[command()]
    Build {
        name: String,
    },

    #[command()]
    Init {
        name: String,
    },

    #[command()]
    Check {
        name: String,
    },

    #[command()]
    New {
        name: String,
    },

    #[command()]
    UpdateRepo {
        repo_file: String,
        package_file: String,
    }
}

fn check_package(package: String) -> bool {
    let current_dir = std::env::current_dir().unwrap_or_else(|err| {
        println!("Error while getting current directory: {}", err);
        std::process::exit(1);
    });

    let package_path = std::path::Path::new(&package);

    if !package_path.exists() {
        return false;
    }

    let package_file = std::fs::read_to_string(package_path.join("info.yaml")).unwrap_or_else(|err| {
        println!("Error while reading package file: {}", err);
        std::process::exit(1);
    });

    let _package = comet::package::Package::from_string(package_file);

    // if there was an error, we wouldn't be here
    // so we can say that the info.yaml file exists and is valid

    // now check if the package has an install file and a remove file
    let install_file = package_path.join("install");
    let remove_file = package_path.join("remove");

    if !install_file.exists() || !remove_file.exists() {
        return false;
    }

    // now make sure theres a "package" directory
    let package_dir = package_path.join("package");

    if !package_dir.exists() {
        return false;
    }

    // we can assume that the package is valid

    // go back to the original directory
    std::env::set_current_dir(current_dir).unwrap_or_else(|err| {
        println!("Error while entering directory: {}", err);
        std::process::exit(1);
    });

    true
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            // create a directory with the name

            // create the directory
            std::fs::create_dir(&name).unwrap_or_else(|err| {
                println!("Error while creating directory: {}", err);
                std::process::exit(1);
            });

            // enter the directory
            std::env::set_current_dir(&name).unwrap_or_else(|err| {
                println!("Error while entering directory: {}", err);
                std::process::exit(1);
            });

            // now create the package folder inside the directory
            std::fs::create_dir("package").unwrap_or_else(|err| {
                println!("Error while creating package directory: {}", err);
                std::process::exit(1);
            });

            // touch the "install" and "remove" files
            std::fs::File::create("install").unwrap_or_else(|err| {
                println!("Error while creating install file: {}", err);
                std::process::exit(1);
            });

            std::fs::File::create("remove").unwrap_or_else(|err| {
                println!("Error while creating remove file: {}", err);
                std::process::exit(1);
            });

            let package = comet::package::Package::new(name.clone(),
                                                       String::from("A package"),
                                                       String::from("1.0.0"),
                                                       std::collections::HashMap::new(),
                                                       String::from("MIT"),
                                                       vec![String::from("")]);

            // write the package to the package file
            std::fs::write("info.yaml", package.to_string()).unwrap_or_else(|err| {
                println!("Error while writing package file: {}", err);
                std::process::exit(1);
            });
        },

        Commands::Build { name } => {
            let current_dir = std::env::current_dir().unwrap_or_else(|err| {
                println!("Error while getting current directory: {}", err);
                std::process::exit(1);
            });

            // check if the package exists by looking for the "name" directory
            let package_path = std::path::Path::new(&name);

            if !package_path.exists() {
                println!("Package does not exist");
                std::process::exit(1);
            }

            // check if the package is valid
            if !check_package(name.clone()) {
                println!("Package is not valid");
                std::process::exit(1);
            }

            // now we can build the package
            // we can use the tar library to create a tarball of the package directory
            let tarball = std::fs::File::create(format!("{}.star", name)).unwrap_or_else(|err| {
                println!("Error while creating tarball: {}", err);
                std::process::exit(1);
            });

            // lets go into the package directory
            std::env::set_current_dir(package_path).unwrap_or_else(|err| {
                println!("Error while entering directory: {}", err);
                std::process::exit(1);
            });

            // lets load the package info.yml file (for later)
            let mut package_file = Package::from_file("info.yaml".to_string());

            let mut a = tar::Builder::new(tarball);

            a.append_dir_all("", ".").unwrap_or_else(|err| {
                println!("Error while appending directory: {}", err);
                std::process::exit(1);
            });

            // remove the '.' directory from the tarball

            a.finish().unwrap_or_else(|err| {
                println!("Error while finishing tarball: {}", err);
                std::process::exit(1);
            });

            let mut hasher = Sha256::new();

            // go back to the original directory
            std::env::set_current_dir(current_dir).unwrap_or_else(|err| {
                println!("Error while entering directory: {}", err);
                std::process::exit(1);
            });

            let mut file = File::open(format!("{}.star", name)).unwrap_or_else(|err| {
                println!("Error while opening tarball: {}", err);
                std::process::exit(1);
            });

            // grab the bytes from the file
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap_or_else(|err| {
                println!("Error while reading tarball: {}", err);
                std::process::exit(1);
            });

            hasher.update(buffer);

            let checksum = format!("{:x}", hasher.finalize());

            // now we can update the package file
            package_file.checksum = Some(checksum.clone());

            // output the package file contents for remote repositories to use
            println!("The following is the package file contents for remote repositories to use:");
            println!("{}", package_file.to_string());
        },

        Commands::Check { name } => {
            // check if the package exists by looking for the "name" directory
            let package_path = std::path::Path::new(&name);

            if !package_path.exists() {
                println!("Package does not exist");
                std::process::exit(1);
            }

            // check if the package is valid
            if !check_package(name.clone()) {
                println!("Package is not valid");
                std::process::exit(1);
            }

            println!("Package is valid");
        },

        Commands::New { name } => {
            // create the directory
            std::fs::create_dir(&name).unwrap_or_else(|err| {
                println!("Error while creating directory: {}", err);
                std::process::exit(1);
            });

            // enter the directory
            std::env::set_current_dir(&name).unwrap_or_else(|err| {
                println!("Error while entering directory: {}", err);
                std::process::exit(1);
            });

            // create an empty package folder, install file, and remove file
            std::fs::create_dir("package").unwrap_or_else(|err| {
                println!("Error while creating package directory: {}", err);
                std::process::exit(1);
            });

            std::fs::File::create("install").unwrap_or_else(|err| {
                println!("Error while creating install file: {}", err);
                std::process::exit(1);
            });

            std::fs::File::create("remove").unwrap_or_else(|err| {
                println!("Error while creating remove file: {}", err);
                std::process::exit(1);
            });

            // now lets create the package file
            // get user input for each field besides the name and checksum
            println!("Enter the description of the package:");
            let mut description = String::new();
            std::io::stdin().read_line(&mut description).unwrap_or_else(|err| {
                println!("Error while reading input: {}", err);
                std::process::exit(1);
            });

            println!("Enter the version of the package:");
            let mut version = String::new();
            std::io::stdin().read_line(&mut version).unwrap_or_else(|err| {
                println!("Error while reading input: {}", err);
                std::process::exit(1);
            });

            println!("Enter the license of the package:");
            let mut license = String::new();

            std::io::stdin().read_line(&mut license).unwrap_or_else(|err| {
                println!("Error while reading input: {}", err);
                std::process::exit(1);
            });

            println!("Does the package have any dependencies? (y/n)");
            let mut dependencies_check = String::new();
            std::io::stdin().read_line(&mut dependencies_check).unwrap_or_else(|err| {
                println!("Error while reading input: {}", err);
                std::process::exit(1);
            });

            let mut dependencies: HashMap<String, String> = std::collections::HashMap::new();

            while dependencies_check.trim() == "y" {
                println!("Enter the dependency name and version in the format \"name:version\" (without quotes)");
                let mut dependency = String::new();
                std::io::stdin().read_line(&mut dependency).unwrap_or_else(|err| {
                    println!("Error while reading input: {}", err);
                    std::process::exit(1);
                });

                dependency = dependency.trim().to_string();

                let dependency_split: Vec<&str> = dependency.split(":").collect();

                if dependency_split.len() != 2 {
                    println!("Invalid dependency format");
                } else {
                    dependencies.insert(dependency_split[0].to_string(), dependency_split[1].to_string());
                }

                // now ask if there are any more dependencies
                println!("Are there any more dependencies? (y/n)");
                dependencies_check = String::new();
                std::io::stdin().read_line(&mut dependencies_check).unwrap_or_else(|err| {
                    println!("Error while reading input: {}", err);
                    std::process::exit(1);
                });
            }

            // do the same for the authors
            println!("Does the package have any authors? (y/n)");
            let mut authors_check = String::new();

            std::io::stdin().read_line(&mut authors_check).unwrap_or_else(|err| {
                println!("Error while reading input: {}", err);
                std::process::exit(1);
            });

            let mut authors: Vec<String> = Vec::new();

            while authors_check.trim() == "y" {
                println!("Enter the author name");
                let mut author = String::new();
                std::io::stdin().read_line(&mut author).unwrap_or_else(|err| {
                    println!("Error while reading input: {}", err);
                    std::process::exit(1);
                });

                authors.push(author.trim().to_string());

                // now ask if there are any more authors
                println!("Are there any more authors? (y/n)");
                authors_check = String::new();
                std::io::stdin().read_line(&mut authors_check).unwrap_or_else(|err| {
                    println!("Error while reading input: {}", err);
                    std::process::exit(1);
                });
            }

            // now we can create the package file
            let package = comet::package::Package::new(name.clone(),
                                                       description.trim().to_string(),
                                                       version.trim().to_string(),
                                                       dependencies.clone(),
                                                       license.trim().to_string(),
                                                       authors.clone());

            // write the package to the package file
            std::fs::write("info.yml", package.to_string()).unwrap_or_else(|err| {
                println!("Error while writing package file: {}", err);
                std::process::exit(1);
            });

            println!("Package created successfully. You can now add the package files to the package directory and build the package with \"startools build {}\"", name);
        }

        Commands::UpdateRepo {repo_file, package_file } => {
            // load the repo file
            let mut repo = comet::repo::Repository::from_file(repo_file.clone());

            // load the package tar file and grab the package file
            let mut package_tar = tar::Archive::new(std::fs::File::open(package_file.clone()).unwrap_or_else(|err| {
                println!("Error while opening package file: {}", err);
                std::process::exit(1);
            }));

            // package file is called "info.yaml", so filter for that

            let files = package_tar.entries().unwrap_or_else(|e| {
                println!("Error while reading package file: {}", e);
                std::process::exit(1);
            });

            let mut package_file = files.filter("info.yaml").next().unwrap_or_else(|| {
                println!("Error while reading package file");
                std::process::exit(1);
            });

            let mut package_file_contents = String::new();

            let _ = package_file.read_to_string(&mut package_file_contents).unwrap_or_else(|e| {
                println!("Error while reading package file: {}", e);
                std::process::exit(1);
            });

            let mut package = comet::package::Package::from_string(package_file_contents);

            // now run a checksum on the package file and set the checksum field to that
            let mut hasher = Sha256::new();

            hasher.update(package_file_contents.as_bytes());
            let checksum = format!("{:x}", hasher.finalize());
            package.checksum = Some(checksum.clone());

            // now add the package to the repo
            repo.add_package(package);

            // now write the repo file
            repo.to_file(repo_file.clone());

            println!("Package added successfully");
        }
    }
}
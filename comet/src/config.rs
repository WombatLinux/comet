
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub repositories: Vec<String>,
    pub keep_package_files: bool,
    pub storage_dir: String,
    pub tmp_dir: String,
}

impl Config {
    pub fn new(repositories: Vec<String>, keep_package_files: bool, storage_dir: String, tmp_dir: String) -> Config {
        Config {
            repositories,
            keep_package_files,
            storage_dir,
            tmp_dir,
        }
    }

    pub fn to_string(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    pub fn from_file() -> Result<Config, String> {
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

        let config = std::fs::read_to_string(config_location.clone()).unwrap();

        serde_yaml::from_str(&config).unwrap_or_else(|_err| {
            crate::setup();

            // try again
            let config = std::fs::read_to_string(config_location.clone()).unwrap();

            serde_yaml::from_str(&config).unwrap_or_else(|err| {
                return Err(format!("Error while parsing config file: {}", err));
            })
        })
    }
}
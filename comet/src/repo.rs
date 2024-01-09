use std::collections::HashMap;
use std::fs::File;
use std::io::{Write};
use serde::{Deserialize, Serialize};
use crate::package::Package;

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub packages: HashMap<String, Package>
}

impl Repository {
    pub fn new(empty: bool) -> Repository {
        if empty {
            return Repository {
                packages: HashMap::new()
            }
        } else {
            let mut base_packages = HashMap::new();
            let mut base_dependencies = HashMap::new();
            base_dependencies.insert("system".to_string(), "1.0.0".to_string());
            base_packages.insert("system".to_string(), Package::new("system".to_string(),
                                                                    "The base system for Wombat Linux".to_string(),
                                                                    "1.0.0".to_string(),
                                                                    HashMap::new(),
                                                                    "MIT".to_string(),
                                                                    vec!["Wombat Linux".to_string()]));
            base_packages.insert("comet".to_string(), Package::new("comet".to_string(),
                                                                   "The package manager for Wombat Linux".to_string(),
                                                                   "1.0.0".to_string(),
                                                                   base_dependencies.clone(),
                                                                   "MIT".to_string(),
                                                                   vec!["Wombat Linux".to_string()]));
            base_packages.insert("startools".to_string(), Package::new("startools".to_string(),
                                                                       "A utility for managing stars".to_string(),
                                                                       "1.0.0".to_string(),
                                                                       base_dependencies.clone(),
                                                                       "MIT".to_string(),
                                                                       vec!["Wombat Linux".to_string()]));

            Repository {
                packages: base_packages
            }
        }
    }

    pub fn add_package(&mut self, package: Package) {
        // if the package is already in the repository, remove it
        if self.packages.contains_key(&package.name) {
            self.packages.remove(&package.name);
        }

        // insert the new package
        self.packages.insert(package.name.clone(), package);
    }

    pub fn remove_package(&mut self, package: String) {
        self.packages.remove(&package);
    }

    pub fn get_package(&self, package: String) -> Option<&Package> {
        self.packages.get(&package)
    }

    pub fn to_string(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    pub fn to_file(&self, path: String) {
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_string().as_bytes()).unwrap();
    }

    pub fn from_string(repository: String) -> Repository {
        serde_yaml::from_str(&repository).unwrap()
    }

    pub fn from_file(path: String) -> Repository {
        let repository = std::fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&repository).unwrap_or_else(|_| Repository::new(true))
    }

    pub fn from_web(url: String) -> Repository {
        let repo_file = format!("{}/repo.yml", url.clone());
        let repository = reqwest::blocking::get(&repo_file).unwrap().text().unwrap();
        serde_yaml::from_str(&repository).unwrap()
    }

    pub fn is_dependency(&self, package: String) -> bool {
        for (_, p) in self.packages.iter() {
            // key is the name of the dependency
            for (key, _) in p.dependencies.iter() {
                if key == &package {
                    return true;
                }
            }
        }
        false
    }
}
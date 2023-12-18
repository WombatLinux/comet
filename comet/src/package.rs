use std::collections::HashMap;

pub struct SemVer {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub suffix: String
}

impl SemVer {
    pub fn new(major: i32, minor: i32, patch: i32, suffix: String) -> SemVer {
        SemVer {
            major,
            minor,
            patch,
            suffix
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}-{}", self.major, self.minor, self.patch, self.suffix)
    }

    pub fn from_string(version: String) -> SemVer {
        let mut split = version.split(".");
        let major = split.next().unwrap().parse::<i32>().unwrap();
        let minor = split.next().unwrap().parse::<i32>().unwrap();
        let mut split = split.next().unwrap().split("-");
        let patch = split.next().unwrap().parse::<i32>().unwrap();
        let suffix = split.next().unwrap().to_string();
        SemVer {
            major,
            minor,
            patch,
            suffix
        }
    }
}

impl PartialEq for SemVer {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch && self.suffix == other.suffix
    }
}

impl Eq for SemVer {}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.major > other.major {
            Some(std::cmp::Ordering::Greater)
        } else if self.major < other.major {
            Some(std::cmp::Ordering::Less)
        } else if self.minor > other.minor {
            Some(std::cmp::Ordering::Greater)
        } else if self.minor < other.minor {
            Some(std::cmp::Ordering::Less)
        } else if self.patch > other.patch {
            Some(std::cmp::Ordering::Greater)
        } else if self.patch < other.patch {
            Some(std::cmp::Ordering::Less)
        }
        else if self.suffix != "" && other.suffix == "" {
            Some(std::cmp::Ordering::Less)
        }
        else if self.suffix == "" && other.suffix != "" {
            Some(std::cmp::Ordering::Greater)
        }
        else if self.suffix != "" && other.suffix != "" {
            Some(self.suffix.cmp(&other.suffix))
        }
        else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub version: String,
    pub dependencies: HashMap<String, String>,
    pub authors: Vec<String>,
    pub license: String,
    pub checksum: Option<String>
}

impl Package {
    pub fn new(name: String, description: String, version: String, stars: HashMap<String, String>, license: String, authors: Vec<String>) -> Package {
        Package {
            name,
            description,
            version,
            dependencies: stars,
            authors,
            license,
            checksum: None
        }
    }

    pub fn to_string(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    pub fn from_string(package: String) -> Package {
        serde_yaml::from_str(&package).unwrap()
    }

    pub fn from_file(path: String) -> Package {
        let package = std::fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&package).unwrap()
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

impl Eq for Package {}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_version = SemVer::from_string(self.version.clone());
        let other_version = SemVer::from_string(other.version.clone());
        if self.name == other.name {
            Some(self_version.partial_cmp(&other_version).unwrap())
        } else {
            None
        }
    }
}
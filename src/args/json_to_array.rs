// json_to_array.rs

use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, fs, io};

#[path = "./terminal.rs"]
mod terminal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub package_management: PackageManagement,
    pub custom_packages: CustomPackages,
}

#[derive(Debug, Serialize, Deserialize)]
// Do NOT deny unknown fields here, we capture them via #[serde(flatten)]
pub struct PackageManagement {
    pub pre: Vec<String>,
    pub post: Vec<String>,

    // Any other keys under "package_management" (e.g., flatpak, snap, nix, appimage, arch, brew...)
    // are captured here as manager specs.
    #[serde(flatten)]
    pub managers: HashMap<String, ManagerSpec>,
}

// Managers can be either a simple boolean or an object with "enabled" and arbitrary flags.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManagerSpec {
    Bool(bool),
    Object {
        enabled: bool,
        #[serde(flatten)]
        flags: HashMap<String, serde_json::Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
// Do NOT deny unknown fields here, we capture them via #[serde(flatten)]
pub struct CustomPackages {
    pub pre: Vec<String>,
    pub post: Vec<String>,

    // Any other keys (apt, pacstall, flatpak, nix, snap, arch...) become entries in this map.
    #[serde(flatten)]
    pub by_manager: HashMap<String, Vec<String>>,
}

pub fn convert(path: &str) -> Result<Config, io::Error> {
    let json = fs::read_to_string(path)?;

    match serde_json::from_str::<Config>(&json) {
        Ok(config) => {
            println!("{:#?}", config); // print the parsed Config
            Ok(config)
        }
        Err(err) => {
            let msg = format!("❌ JSON schema error at column {}: {}", err.column(), err);
            eprintln!("{}", msg);
            Err(io::Error::new(io::ErrorKind::InvalidData, msg))
        }
    }
}

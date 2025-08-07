use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{fs, io};

#[path = "./terminal.rs"]
mod terminal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub package_management: PackageManagement,
    pub custom_packages: CustomPackages,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageManagement {
    pub pre: Vec<String>,
    pub flatpak: Flatpak,
    pub nix: bool,
    pub snap: bool,
    pub appimage: AppImage,
    pub post: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flatpak {
    pub enabled: bool,
    pub include_beta: bool,
    pub install_flatseal: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppImage {
    pub enabled: bool,
    pub include_appimage_manager: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomPackages {
    pub pre: Vec<String>,
    pub apt: Vec<String>,
    pub pacstall: Vec<String>,
    pub flatpak: Vec<String>,
    pub nix: Vec<String>,
    pub snap: Vec<String>,
    pub post: Vec<String>,
}

pub fn convert(path: &str) -> Result<Config, io::Error> {
    let json = fs::read_to_string(path)?;

    match serde_json::from_str::<Config>(&json) {
        Ok(config) => Ok(config),
        Err(err) => {
            let msg = format!("❌ JSON schema error at column {}: {}", err.column(), err);
            eprintln!("{}", msg);
            Err(io::Error::new(io::ErrorKind::InvalidData, msg))
        }
    }
}

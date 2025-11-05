use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    package_manager: Vec<PackageManager>,
}

#[derive(Debug, Deserialize)]
struct PackageManager {
    id: String,
    check: String,                      // new: presence check
    bootstrap: String,                  // new: install the manager if check fails
    enable: String,                     // now used as setup/enabling step
    install: String,                    // one-by-one template ("... {{package}}")
    others: Option<HashMap<String, String>>, // flag -> description
}

fn main() {
    let toml_str = fs::read_to_string("Packages.toml")
        .expect("Failed to read Packages.toml");

    let config: Config = toml::from_str(&toml_str)
        .expect("Failed to parse TOML");

    for pm in config.package_manager {
        println!("Package manager: {}", pm.id);
        println!("  check: {}", pm.check);
        println!("  bootstrap: {}", pm.bootstrap);
        println!("  enable: {}", pm.enable);
        println!("  install: {}", pm.install);

        if let Some(others) = pm.others {
            if !others.is_empty() {
                println!("  flags:");
                for (key, desc) in others {
                    println!("    {} = {}", key, desc);
                }
            }
        }
    }
}

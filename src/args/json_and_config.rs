use std::{collections::HashMap, fs, io};
use serde::Deserialize;
use serde_json::Value;

// import your JSON parsing types and function
use crate::args::json_to_array::{convert as load_json_config, ManagerSpec};

/// TOML structs (non-binary, library style)
#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub package_manager: Vec<TomlPackageManager>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TomlPackageManager {
    pub id: String,
    pub check: String,                 // presence check command
    pub bootstrap: String,             // install manager if not present
    pub enable: String,                // setup/enabling command (previously "setup")
    pub install: String,               // one-by-one template: "... {{package}}"
    pub others: Option<HashMap<String, String>>, // flag -> description
}

/// Load Packages.toml into a map id -> definition
pub fn load_toml_defs(path: &str) -> Result<HashMap<String, TomlPackageManager>, io::Error> {
    let toml_str = fs::read_to_string(path)?;
    let cfg: TomlConfig = toml::from_str(&toml_str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("TOML parse error: {e}")))?;
    Ok(cfg.package_manager.into_iter().map(|pm| (pm.id.clone(), pm)).collect())
}

/// Strict comparison: returns Ok(report) when valid, Err(message) on the first validation failure.
pub fn compare_and_report(json_path: &str, toml_path: &str) -> Result<String, String> {
    // load TOML defs
    let toml_defs = load_toml_defs(toml_path).map_err(|e| format!("Failed to load TOML: {}", e))?;

    // load JSON config using your existing convert (keeps error messages as io::Error)
    let json_cfg = load_json_config(json_path).map_err(|e| format!("Failed to load JSON: {}", e))?;

    // Validate managers in package_management
    for (json_id, spec) in &json_cfg.package_management.managers {
        // strict: id must exist in TOML exactly
        let def = toml_defs.get(json_id).ok_or_else(|| {
            format!("Unknown manager '{}' in JSON; not defined in Packages.toml", json_id)
        })?;

        match spec {
            ManagerSpec::Bool(_) => {
                // ok: boolean form uses no flags
            }
            ManagerSpec::Object { enabled: _, flags } => {
                if let Some(allowed) = &def.others {
                    for (flag, value) in flags {
                        if !allowed.contains_key(flag) {
                            return Err(format!(
                                "Unknown flag '{}' for manager '{}' in JSON; not defined in Packages.toml",
                                flag, json_id
                            ));
                        }
                        // optional: disallow complex types for flags
                        match value {
                            Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Null => {}
                            Value::Array(_) | Value::Object(_) => {
                                return Err(format!(
                                    "Unsupported complex value for flag '{}' on '{}': {}",
                                    flag, json_id, value
                                ));
                            }
                        }
                    }
                } else if !flags.is_empty() {
                    let unknowns = flags.keys().cloned().collect::<Vec<_>>().join(", ");
                    return Err(format!(
                        "Manager '{}' does not support flags (TOML has no 'others'), but JSON provided: {}",
                        json_id, unknowns
                    ));
                }
            }
        }
    }

    // Validate custom_packages keys reference known managers
    for mgr in json_cfg.custom_packages.by_manager.keys() {
        if !toml_defs.contains_key(mgr) {
            return Err(format!(
                "Unknown custom_packages key '{}' in JSON; must be defined in Packages.toml",
                mgr
            ));
        }
    }

    // Build a short report string
    let mut rpt = String::new();
    rpt.push_str("Validation OK\n\nDefined package managers (TOML):\n");
    for (id, def) in &toml_defs {
        rpt.push_str(&format!(" - {}\n", id));
        rpt.push_str(&format!("    check: {}\n", def.check));
        if !def.bootstrap.trim().is_empty() {
            rpt.push_str(&format!("    bootstrap: {}\n", def.bootstrap));
        }
        if !def.enable.trim().is_empty() {
            rpt.push_str(&format!("    enable: {}\n", def.enable));
        }
        rpt.push_str(&format!("    install: {}\n", def.install));
        if let Some(others) = &def.others {
            if !others.is_empty() {
                let keys = others.keys().cloned().collect::<Vec<_>>().join(", ");
                rpt.push_str(&format!("    flags: {}\n", keys));
            }
        }
    }

    rpt.push_str("\nJSON snapshot (parsed):\n");
    // show json managers and any flags used
    for (id, spec) in &json_cfg.package_management.managers {
        match spec {
            ManagerSpec::Bool(b) => {
                rpt.push_str(&format!(" - {} (enabled: {})\n", id, b));
            }
            ManagerSpec::Object { enabled, flags } => {
                rpt.push_str(&format!(" - {} (enabled: {})\n", id, enabled));
                if !flags.is_empty() {
                    let mut flag_lines: Vec<String> = Vec::new();
                    for (k, v) in flags {
                        flag_lines.push(format!("{}={}", k, v));
                    }
                    rpt.push_str(&format!("    flags: {}\n", flag_lines.join(", ")));
                }
            }
        }
    }

    // custom packages listing
    rpt.push_str("\nCustom packages by manager:\n");
    for (mgr, list) in &json_cfg.custom_packages.by_manager {
        if list.is_empty() {
            rpt.push_str(&format!(" - {}: (none)\n", mgr));
        } else {
            rpt.push_str(&format!(" - {}: {}\n", mgr, list.join(", ")));
        }
    }

    Ok(rpt)
}

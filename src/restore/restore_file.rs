// src/restore/restore_file.rs

use crate::restore::check;
use crate::restore::session;
use crate::args::json_and_config::load_toml_defs;
use chrono::Local;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

pub fn forward(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = dir.to_str().ok_or_else(|| {
        Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "path contains non-UTF-8 characters",
        )) as Box<dyn std::error::Error>
    })?;

    let config = check::json_validation(path_str).map_err(|e| {
        Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("JSON validation failed: {}", e),
        )) as Box<dyn std::error::Error>
    })?;

    // Load TOML definitions (adjust path if Packages.toml lives elsewhere)
    let toml_path = "Packages.toml";
    let toml_defs = load_toml_defs(toml_path).map_err(|e| {
        Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to load TOML definitions from {}: {}", toml_path, e),
        )) as Box<dyn std::error::Error>
    })?;

    // Call session with both JSON config and TOML defs
    println!("DEBUG: about to call start_session");
    session::start_session(&config, &toml_defs);
    println!("DEBUG: returned from start_session");

    Ok(())
}


pub fn generate_directory(directory_name: &str) -> Result<PathBuf, io::Error> {
    let now = Local::now();
    let dir = PathBuf::from("configs").join(format!(
        "{}-{}",
        now.format("%Y-%m-%d-%H-%M-%S"),
        directory_name
    ));

    fs::create_dir_all(&dir)?;

    let absolute = dir.canonicalize()?;
    println!("Created directory: {}", absolute.display());

    Ok(absolute)
}


pub fn remote(link: &str) -> Result<(), Box<dyn std::error::Error>> {
    let sanitized = link
        .replace("http://", "")
        .replace("https://", "")
        .replace('/', "_")
        .replace(".git", "");

    let dir = generate_directory(&sanitized)?;

    let status = Command::new("git")
        .arg("clone")
        .arg(link)
        .arg(&dir)
        .status()?;

    if !status.success() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("git clone failed with exit code: {:?}", status.code()),
        )));
    }

    forward(dir.as_path())?;

    Ok(())
}

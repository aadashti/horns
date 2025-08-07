// src/restore/restore_file.rs

use chrono::Local;
use std::{fs, io, path::PathBuf};

#[path = "../args/terminal.rs"]
mod terminal;

#[path = "check.rs"]
mod check;

#[path = "session.rs"]
mod session;

#[path = "../args/json_to_array.rs"]
mod json_to_array;
use json_to_array::Config;

pub fn generate_directory(directory_name: &str) -> Result<PathBuf, io::Error> {
    let now = Local::now();
    let dir = PathBuf::from("configs").join(format!(
        "{}-{}",
        now.format("%Y-%m-%d-%H-%M-%S"),
        directory_name
    ));

    fs::create_dir_all(&dir)?;

    // Convert to absolute path
    let absolute = dir.canonicalize()?;
    println!("Created directory: {}", absolute.display());

    Ok(absolute)
}

pub fn remote(link: &str) -> bool {
    let sanitized = link
        .replace("http://", "")
        .replace("https://", "")
        .replace("/", "_")
        .replace(".git", "");

    match generate_directory(&sanitized) {
        Ok(dir) => {
            let cmd = format!("git clone {} {}", link, dir.display());

            if let Err(e) = terminal::output(&cmd) {
                eprintln!("git clone failed: {e}");
                return false;
            }
            match check::json_validation(&dir.display().to_string()) {
                Ok(config) => {
                    session::start_session(&config);
                    true
                }
                Err(e) => {
                    eprintln!("[RESTORE FILE] JSON validation failed: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("create_dir_all failed: {e}");
            false
        }
    }
}

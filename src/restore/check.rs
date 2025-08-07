// check.rs

use serde_json::Value;
use std::io;

#[path = "../args/terminal.rs"]
mod terminal;

#[path = "../args/json_to_array.rs"]
mod json_to_array;
use json_to_array::Config;

pub fn json_validation(directory_name: &str) -> Result<Config, io::Error> {
    println!("Scanning: {}", directory_name);

    // Run `ls`, get space-separated output
    let output = terminal::output_access(&format!("ls {}", directory_name))?;
    let files: Vec<_> = output
        .split_whitespace()
        .filter(|f| f.ends_with(".json"))
        .collect();

    match files.as_slice() {
        [] => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No JSON files found",
        )),
        [single] => {
            let full_path = format!("{}/{}", directory_name.trim(), single);
            println!("✅ Found JSON: {full_path}");

            let data = json_to_array::convert(&full_path)?;

            Ok(data)
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "More than one JSON file found",
        )),
    }
}

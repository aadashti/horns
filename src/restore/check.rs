use crate::args::json_to_array;
use crate::args::json_to_array::Config;
use crate::args::terminal;
use crate::args::json_and_config;
use std::io;

pub fn json_validation(directory_name: &str) -> Result<Config, io::Error> {
    println!("Scanning: {}", directory_name);

    // Run `ls`, get space-separated output
    let output = terminal::output_access(&format!("ls {}", directory_name))?;
    let files: Vec<_> = output
        .split_whitespace()
        .filter(|f| f.ends_with(".json"))
        .collect();

    match files.as_slice() {
        [] => Err(io::Error::new(io::ErrorKind::InvalidInput, "No JSON files found")),
        [single] => {
            // Build the full path to the single JSON file we found
            let full_path = format!("{}/{}", directory_name.trim_end_matches('/'), single);
            println!("✅ Found JSON: {full_path}");

            // Validate JSON vs TOML and print the report
            let report = json_and_config::compare_and_report(&full_path, "args/Packages.toml")
                .map_err(|e| {
                    eprintln!("Validation error: {}", e);
                    io::Error::new(io::ErrorKind::Other, e)
                })?;
            println!("{}", report);

            // Parse JSON into Config and return it
            let config = json_to_array::convert(&full_path)?;
            Ok(config)
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "More than one JSON file found",
        )),
    }
}

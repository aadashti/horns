// src/restore/session.rs
use crate::args::json_to_array::Config;

pub fn start_session(config: &Config) {
    let config = config;
    println!("Starting session…");

    if !config.package_management.pre.is_empty() {
        println!("⏱ Running pre-package scripts:");
        for script in &config.package_management.pre {
            println!("→ {}", script);
        }
    }
}

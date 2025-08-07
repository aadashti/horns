// src/restore/session.rs
use serde_json::Value;

#[path = "../args/json_to_array.rs"]
mod json_to_array;
use json_to_array::Config;

pub fn start_session(config: &Config) {
    let config = config;
    println!("Starting session…");

    println!("⏱ Running pre-package scripts:");
    for script in &config.package_management.pre {
        println!("→ {}", script);
        // terminal::output_access(script).unwrap();
    }

    println!("\n🔍 Package Managers:");
    println!(
        "Flatpak enabled? {}",
        config.package_management.flatpak.enabled
    );
    println!("Nix enabled?     {}", config.package_management.nix);
    println!("Snap enabled?    {}", config.package_management.snap);
    println!(
        "AppImage enabled? {}",
        config.package_management.appimage.enabled
    );

    println!("\n📦 Installing Custom Packages:");

    println!("APT:");
    for pkg in &config.custom_packages.apt {
        println!("→ sudo apt install {}", pkg);
    }

    println!("Flatpak:");
    for pkg in &config.custom_packages.flatpak {
        println!("→ flatpak install {}", pkg);
    }

    // Add sections for pacstall, nix, snap...

    println!("\n🧹 Running post-package scripts:");
    for script in &config.package_management.post {
        println!("→ {}", script);
    }
}

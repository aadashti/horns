// TEMPLATE FILE - TO BE COMPLETED

use crate::args::json_to_array::Config;
use crate::args::pm::flatpak;

pub fn start_session(config: &Config) {
    println!("Starting session…");

    for script in &config.package_management.pre {
        println!("→ {}", script);
    }

    let fp = &config.package_management.flatpak;
    flatpak::flatpak_access(
        &fp.enabled,
        &fp.include_beta,
        &fp.install_flatseal,
        &config.custom_packages.flatpak,
    );


    for script in &config.package_management.post {
        println!("→ {}", script);
    }

    println!("Session complete ✅");
}

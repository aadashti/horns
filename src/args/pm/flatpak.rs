// TEMPLATE FILE - TO BE COMPLETED

use crate::args::terminal;

pub fn flatpak_access(
    enabled: &bool,
    include_beta: &bool,
    install_flatseal: &bool,
    packages: &Vec<String>,
) {
    if !*enabled {
        println!("🚫 Flatpak is disabled in the configuration.");
        return;
    }

    // Check if Flatpak is installed
    let installed = match terminal::output_access("flatpak --version") {
        Ok(output) => !output.trim().is_empty(),
        Err(_) => false,
    };

    if !installed {
        println!("⚠️  Flatpak is not installed on this system!");
        return;
    }

    println!(
        "✅ Flatpak detected: {}",
        terminal::output_access("flatpak --version")
            .unwrap_or_else(|_| "unknown version".to_string())
    );

    // Handle beta repositories
    if *include_beta {
        println!("🧪 Including beta repositories…");
        // Example: terminal::execute("flatpak remote-add --if-not-exists flathub-beta https://flathub.org/beta-repo/flathub-beta.flatpakrepo");
    }

    // Handle Flatseal installation
    if *install_flatseal {
        println!("🔧 Installing Flatseal…");
        // Example: terminal::execute("flatpak install -y flathub com.github.tchx84.Flatseal");
    }

    // Handle custom Flatpak packages
    if !packages.is_empty() {
        println!("📦 Installing custom Flatpak packages:");
        for pkg in packages {
            println!("  → {}", pkg);
            // Example: terminal::execute(&format!("flatpak install -y flathub {}", pkg));
        }
    } else {
        println!("ℹ️  No additional Flatpak packages specified.");
    }
}

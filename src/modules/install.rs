use std::process::Command;

/// Runs a shell command and returns whether it succeeded.
fn run(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// package manager (`pm`)
/// package (`pkg`)
pub fn pmc(ask: bool, pm: &str, pkg: &str) -> bool {
    let cmd = match pm {
        "apt" => {
            // `-y` only if ask == false
            let flag = if ask { "" } else { "-y" };
            format!("sudo apt install {} {}", pkg, flag)
                .trim_end()
                .to_string()
        }
        "pacstall" => {
            if ask {
                format!("pacstall -I {}", pkg)
            } else {
                format!("pacstall -I {} -P", pkg)
            }
        }
        "flatpak" => {
            let flag = if ask { "" } else { "-y" };
            format!("flatpak install flathub {} {}", pkg, flag)
                .trim_end()
                .to_string()
        }
        "snapcraft" => {
            format!("sudo snap install {}", pkg)
        }
        other => {
            eprintln!("Unknown package manager `{}`", other);
            return false;
        }
    };

    if !run(&cmd) {
        eprintln!("Failed to install `{}` via `{}`", pkg, pm);
        false
    } else {
        println!("Installed `{}` successfully with `{}`", pkg, pm);
        true
    }
}

pub fn install_tool(title: &str, package_string: &str, ask_str: &str) -> bool {
    let ask = ask_str == "true";
    let mut parts = package_string.split_whitespace();
    let package_manager = match parts.next() {
        Some(pm) => pm,
        None => {
            eprintln!("No package manager specified for `{}`", title);
            return false;
        }
    };
    let packages: Vec<&str> = parts.collect();

    println!(
        "Installing `{}` with `{}`: {:?}",
        title, package_manager, packages
    );

    for pkg in packages {
        println!("\x1B[32m[Installing {} >> {}]\x1B[0m", pkg, title);
        if !pmc(ask, package_manager, pkg) {
            return false;
        }
    }

    true
}

use std::process::Command;

fn run(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn install_tool(title: &str, package_string: &str, ask: &str) -> bool {
    let package_list: Vec<&str> = package_string.split_whitespace().collect();
    println!("{} Packages to install: {:?}", package_list);
    println!("{} Ask prompt: {}", ask);

    for package in package_list {
        let tag = format!("\x1B[32m{}\x1B[0m", format!("[Installing {} >> {}]", package, todo.title));
        let cmd = if ask == "true" {
            format!("sudo apt install {}", package)
        } else {
            format!("sudo apt install {} -y", package)
        };

        if !run(&cmd) {
            eprintln!("Failed to install `{}`", package);
            return false;
        } else {
            println!("`{}` has been installed successfully", package);
            return true;
        }
    }

    true
}

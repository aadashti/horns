use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn run_with_tag(cmd: &str, tag: &str) -> bool {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let mut out_reader = BufReader::new(stdout).lines();
    let mut err_reader = BufReader::new(stderr).lines();

    while let Some(Ok(line)) = out_reader.next() {
        println!("{} {}", tag, line);
    }
    while let Some(Ok(line)) = err_reader.next() {
        eprintln!("{} {}", tag, line);
    }

    child.wait().map(|s| s.success()).unwrap_or(false)
}

pub fn install_tool(title: &str, package_string: &str, ask: &str) -> bool {
    let package_list: Vec<&str> = package_string.split_whitespace().collect();
    println!("Packages to install: {:?}", package_list);
    println!("Ask prompt: {}", ask);

    for package in package_list {
        let tag = format!(
            "\x1B[32m[Installing {} ({})]\x1B[0m",
            package, title
        );
        println!("{}", tag);

        let cmd = if ask == "true" {
            format!("sudo rhino-pkg install {}", package)
        } else {
            format!("sudo rhino-pkg install {} -y", package)
        };

        if !run_with_tag(&cmd, &tag) {
            eprintln!("{} Failed to install `{}`", tag, package);
            return false;
        }
    }

    true
}

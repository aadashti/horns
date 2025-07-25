use std::io::{self, Write};
use std::process::Command;

fn run(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn yes_or_no(prompt: &str) -> bool {
    print!("{} [y/N] ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

pub fn script_tool(title: &str, script: &str, ask: &str) -> bool {
    // println!("Title: {}", title);
    // println!("Script: {}", script);

    if ask == "true" {
        let question = format!("Execute script '{}'?", title);
        if !yes_or_no(&question) {
            println!("Skipping…");
            return true;
        }
    }

    let tag = format!("\x1B[32m[Running Scripts>> {}]\x1B[0m", title);
    println!("{}", tag);

    let success = run(script);
    if success {
        println!("Script completed successfully");
    } else {
        eprintln!("Script failed");
    }
    success
}
use std::{
    io::{self, Write, BufRead, BufReader},
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

pub fn yes_or_no(prompt: &str) -> bool {
    print!("{} [y/N] ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

pub fn script_tool(title: &str, script: &str, ask: &str) -> bool {
    println!("Title: {}", title);
    println!("Script: {}", script);

    if ask == "true" {
        let question = format!("Execute script '{}'?", title);
        if !yes_or_no(&question) {
            println!("Skipping…");
            return true;
        }
    }

    let tag = format!("\x1B[32m[Running Script ({})]\x1B[0m", title);
    println!("{}", tag);

    let success = run_with_tag(script, &tag);
    if success {
        println!("{} Script completed successfully", tag);
    } else {
        eprintln!("{} Script failed", tag);
    }
    success
}

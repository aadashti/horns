// src/restore/session.rs

use std::collections::HashMap;
use serde_json::Value;

use crate::args::json_to_array::{Config, ManagerSpec};
use crate::args::json_and_config::TomlPackageManager;

#[path = "../args/terminal.rs"]
mod terminal;

/// Start session: fully dynamic, consults TOML for what to run.
/// Flags are applied (via placeholder expansion) before bootstrap when required.
pub fn start_session(config: &Config, toml_defs: &HashMap<String, TomlPackageManager>) {
    println!("=== SESSION START (dynamic) ===");

    // Run pre scripts
    for script in &config.package_management.pre {
        let s = script.trim();
        if s.is_empty() { continue; }
        info("pre", s);
        let _ = run_cmd(s);
    }

    // For each manager requested in JSON
    for (mgr_id, spec) in &config.package_management.managers {
        // Is it enabled in JSON?
        if !is_enabled(spec) {
            println!("(skip) manager '{}' disabled in JSON", mgr_id);
            continue;
        }

        println!("\nknock knock → '{}'", mgr_id);

        // Lookup TOML definition
        let def = match toml_defs.get(mgr_id) {
            Some(d) => d,
            None => {
                eprintln!("(error) unknown manager '{}' — not defined in Packages.toml; skipping", mgr_id);
                continue;
            }
        };

        // Show the check string from TOML
        if !def.check.trim().is_empty() {
            println!("json_and_config: wait, I'll check it for you");
            println!("→ check command: {}", def.check);
        }

        // Run initial check (treat empty check as present)
        let mut present = run_check(&def.check);
        if present {
            println!("json_and_config: yup — installed ✅");
        } else {
            println!("json_and_config: nope — not installed ❌");
            // If there's an enable command, run it now (before flags/bootstrap)
            if !def.enable.trim().is_empty() {
                println!("-> running enable for '{}': {}", mgr_id, def.enable);
                let _ = run_cmd(&def.enable);
            }
        }

        // Gather flags from JSON (owned map so we can pass a reference easily)
        let flags_map: HashMap<String, Value> = match spec {
            ManagerSpec::Bool(_) => HashMap::new(),
            ManagerSpec::Object { flags, .. } => flags.clone(),
        };

        // IMPORTANT: apply flag-driven enable-like actions that SHOULD run before bootstrap.
        // Expand enable using flags and run if it produced a changed/meaningful command.
        let flagged_enable = expand_with_flags(&def.enable, &flags_map, None);
        if should_run_expanded(&flagged_enable, &def.enable) {
            println!("-> running flagged enable for '{}': {}", mgr_id, flagged_enable);
            let _ = run_cmd(&flagged_enable);
        }

        // Re-check after enable + flagged-enable
        present = run_check(&def.check);

        // If still missing, attempt bootstrap (expanded with flags first)
        if !present && !def.bootstrap.trim().is_empty() {
            let flagged_bootstrap = expand_with_flags(&def.bootstrap, &flags_map, None);
            let to_run = if should_run_expanded(&flagged_bootstrap, &def.bootstrap) {
                flagged_bootstrap
            } else {
                def.bootstrap.clone()
            };
            println!("-> attempting bootstrap for '{}': {}", mgr_id, to_run);
            let _ = run_cmd(&to_run);
        }

        // Final check
        present = run_check(&def.check);
        if !present {
            eprintln!(
                "(warn) '{}' still not available after enable/bootstrap — skipping installs",
                mgr_id
            );
            continue;
        }

        // Install packages one-by-one (expand {{package}} and any {{flag}} placeholders)
        let pkgs = config
            .custom_packages
            .by_manager
            .get(mgr_id)
            .cloned()
            .unwrap_or_default();

        if pkgs.is_empty() {
            println!("(info) no packages for '{}'", mgr_id);
            continue;
        }

        println!("(info) installing {} package(s) via '{}'", pkgs.len(), mgr_id);
        for pkg in pkgs {
            // Expand flags and package in one pass
            let cmd = expand_with_flags(&def.install, &flags_map, Some(("package", &pkg)));
            println!("-> [{}] {}", mgr_id, cmd);
            let _ = run_cmd(&cmd);
        }
    }

    // Run post scripts
    for script in &config.package_management.post {
        let s = script.trim();
        if s.is_empty() { continue; }
        info("post", s);
        let _ = run_cmd(s);
    }

    println!("=== SESSION COMPLETE ===");
}

/// Small helper: whether ManagerSpec is enabled
fn is_enabled(spec: &ManagerSpec) -> bool {
    match spec {
        ManagerSpec::Bool(b) => *b,
        ManagerSpec::Object { enabled, .. } => *enabled,
    }
}

/// Run the check command string; empty = treat as present (true).
/// Uses terminal::output_access and prints captured stdout/stderr for debug.
fn run_check(check: &str) -> bool {
    let c = check.trim();
    if c.is_empty() {
        println!("(check) empty check string — treating as present");
        return true;
    }

    println!("(check) running: {}", c);
    match terminal::output_access(c) {
        Ok(stdout) => {
            if !stdout.trim().is_empty() {
                println!("(check stdout) {}", stdout.trim_end());
            } else {
                println!("(check) command returned success with no stdout");
            }
            println!("(check) considered present (exit 0)");
            true
        }
        Err(stderr) => {
            eprintln!("(check stderr) {}", stderr.to_string());
            println!("(check) considered not present (non-zero exit)");
            false
        }
    }
}

/// Run a shell command string using terminal::output_access so we can capture and print output.
/// Returns true on success, false on failure.
fn run_cmd(cmd: &str) -> bool {
    let c = cmd.trim();
    if c.is_empty() {
        println!("(run_cmd) empty command, skipping");
        return true;
    }
    println!("$ {}", c);

    match terminal::output_access(c) {
        Ok(stdout) => {
            if !stdout.trim().is_empty() {
                println!("(out) {}", stdout.trim_end());
            }
            println!("(run_cmd) succeeded");
            true
        }
        Err(stderr) => {
            eprintln!("(err) {}", stderr.to_string());
            println!("(run_cmd) failed");
            false
        }
    }
}

/// Replace placeholders {{key}} from flags map. If `extra` is Some(("package","name")) it will also replace {{package}}.
fn expand_with_flags(template: &str, flags: &HashMap<String, Value>, extra: Option<(&str, &str)>) -> String {
    let mut out = template.to_string();
    // Replace flags (stringify values)
    for (k, v) in flags {
        let placeholder = format!("{{{{{}}}}}", k);
        let replacement = match v {
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::Null => "".to_string(),
            Value::Array(_) | Value::Object(_) => v.to_string(),
        };
        out = out.replace(&placeholder, &replacement);
    }
    // Extra placeholder (e.g., package)
    if let Some((k, val)) = extra {
        let placeholder = format!("{{{{{}}}}}", k);
        out = out.replace(&placeholder, val);
    }
    out
}

/// Decide whether an expanded command should be run: non-empty and different from original.
fn should_run_expanded(expanded: &str, original: &str) -> bool {
    let e = expanded.trim();
    let o = original.trim();
    !e.is_empty() && e != o
}

/// Simple info-style print helper
fn info(stage: &str, s: &str) {
    println!("> {}: {}", stage, s);
}

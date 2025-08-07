use std::env;

#[path = "restore/restore_file.rs"]
mod restore;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // ---------------------------------I/O---------------------------------
    match args.as_slice() {
        // Backup
        [cmd, path] if cmd.as_str() == "backup" => {
            println!("Backup command with path: {}", path);
        }

        // Restore command with path
        [cmd, opt, path]
            if cmd.as_str() == "restore" && (opt.as_str() == "--path" || opt.as_str() == "-p") =>
        {
            println!("Restore from path: {}", path);
        }

        // Restore command with remote
        [cmd, opt, repo]
            if cmd.as_str() == "restore"
                && (opt.as_str() == "--remote" || opt.as_str() == "-r") =>
        {
            restore::remote(repo);
        }

        // Restore command with Backup ID
        [cmd, id] if cmd.as_str() == "restore" => {
            println!("Default restore with ID: {}", id);
        }

        _ => {
            println!("Invalid command format");
            println!("Usage examples:");
            println!("  backup /some/path");
            println!("  restore --remote myrepo");
            println!("  restore -p /backup/path");
            println!("  restore backup123");
        }
    }
    // ---------------------------------I/O---------------------------------
}

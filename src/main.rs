mod todos;

use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(name = "horns")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Restore a config from a JSON file
    ConfigRestore {
        /// Path to your JSON config
        path: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ConfigRestore { path } => {
            let todos = todos::read_todos(&path)?;
            todos::print_todos(&todos);
        }
    }

    Ok(())
}

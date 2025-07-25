mod todos;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: cargo run config-restore <path_to_json>");

    let todos = todos::read_todos(&path)?;

    todos::print_todos(&todos);

    Ok(())
}

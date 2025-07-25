use serde::Deserialize;
use std::{error::Error, fs, path::Path};
#[path = "modules/install.rs"]
mod install;
#[path = "modules/script.rs"]
mod script;

#[derive(Debug, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub detail: String,
    #[serde(default)]
    pub ask: String,
}


pub fn read_todos<P: AsRef<Path>>(path: P) -> Result<Vec<Todo>, Box<dyn Error>> {
    let json = fs::read_to_string(path)?;
    let todos = serde_json::from_str(&json)?;
    Ok(todos)
}


pub fn print_todos(todos: &[Todo]) {
    for todo in todos {
        println!(
            "id={} title={} detail={} ask={}",
            todo.id, todo.title, todo.detail, todo.ask
        );

        if todo.id == "install" {
            if !install::install_tool(&todo.title, &todo.detail, &todo.ask) {
                eprintln!("A process failed to run, skipping…");
            }
        }

        if todo.id == "script" {
            if !script::script_tool(&todo.title, &todo.detail, &todo.ask) {
                eprintln!("A process failed to run, skipping…");
            }
        }
    }
}

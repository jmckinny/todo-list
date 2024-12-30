mod commands;
mod todo_list;
use std::{env, path::Path};

use todo_list::TodoList;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = get_todo_list()?;
    run_command(&args, &mut todo_list)?;
    Ok(())
}

fn get_todo_list() -> Result<TodoList, std::io::Error> {
    let home_todo = format!("{}/.todo", env::var("HOME").expect("$HOME not set"));
    let default_todofile = Path::new(&home_todo);
    let todo_file = get_todo_file(default_todofile);

    let todo_file_data = std::fs::read_to_string(&todo_file)?;
    let todo_list = todo_file_data.parse()?;
    Ok(todo_list)
}

fn get_todo_file(default: &Path) -> &Path {
    let cur_dir = Path::new(".todo");
    if cur_dir.exists() {
        return cur_dir;
    }
    default
}

fn run_command(args: &[String], todo_list: &mut TodoList) -> Result<(), std::io::Error> {
    let command: &str = args.get(1).map(String::as_str).unwrap_or("list");
    match command {
        "list" => {
            commands::list::list_items(todo_list);
        }
        _ => {
            eprintln!("Unrecognized command: '{}'", command)
        }
    }
    Ok(())
}

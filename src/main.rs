mod commands;
mod todo_error;
mod todo_list;
use std::{
    env,
    io::{self, Write},
    path::Path,
};

use todo_error::TodoError;
use todo_list::TodoList;

fn main() -> Result<(), TodoError> {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = get_todo_list()?;
    run_command(&args, &mut todo_list)?;
    Ok(())
}

fn get_todo_list() -> Result<TodoList, TodoError> {
    let home_todo = format!("{}/.todo", env::var("HOME").expect("$HOME not set"));
    let default_todofile = Path::new(&home_todo);
    let mut todo_file = get_todo_file(default_todofile);

    if !todo_file.exists() {
        let local = Path::new(".todo");
        let create_new_todo = prompt_create_new_todo_list();
        if !create_new_todo {
            eprintln!("Aborting");
            std::process::exit(1);
        }
        std::fs::File::create_new(local)?;
        todo_file = local;
    }

    TodoList::from_file(todo_file)
}

fn prompt_create_new_todo_list() -> bool {
    let mut input = String::new();
    print!("No todo file found, create .todo locally? [Y/n] ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    if input.trim().to_lowercase() == "n" {
        return false;
    }
    true
}

fn get_todo_file(default: &Path) -> &Path {
    let cur_dir = Path::new(".todo");
    if cur_dir.exists() {
        return cur_dir;
    }
    default
}

fn run_command(args: &[String], todo_list: &mut TodoList) -> Result<(), TodoError> {
    let command: &str = args.get(1).map(String::as_str).unwrap_or("list");
    match command {
        "l" | "list" => {
            commands::list::list_items(todo_list);
        }
        "a" | "add" => {
            commands::add::add_item(todo_list, args)?;
            commands::list::list_items(todo_list);
        }
        "r" | "remove" => {
            commands::remove::remove_item_index(todo_list, args)?;
            commands::list::list_items(todo_list);
        }
        _ => {
            eprintln!("Unrecognized command: '{}'", command);
            std::process::exit(1);
        }
    }
    Ok(())
}

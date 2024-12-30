mod commands;
mod data;
mod todo_error;
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
};

use data::todo_list::TodoList;
use todo_error::TodoError;

fn main() -> Result<(), TodoError> {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = get_todo_list()?;
    run_command(&args, &mut todo_list)?;
    Ok(())
}

fn get_todo_list() -> Result<TodoList, TodoError> {
    let mut todo_file = get_todo_file();

    if !todo_file.exists() {
        let local = PathBuf::from(".todo");
        let create_new_todo = prompt_create_new_todo_list();
        if !create_new_todo {
            eprintln!("Aborting");
            std::process::exit(1);
        }
        std::fs::File::create_new(&local)?;
        todo_file = local;
    }

    TodoList::from_file(&todo_file)
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

fn get_todo_file() -> PathBuf {
    let home_todo = format!("{}/.todo", env::var("HOME").expect("$HOME not set"));
    let default_todofile = PathBuf::from(home_todo);

    let cur_dir = PathBuf::from(".todo");
    if cur_dir.exists() {
        return cur_dir;
    }
    default_todofile
}

fn save_current_todo_list(todo_list: &TodoList) -> Result<(), std::io::Error> {
    todo_list.save_to_file(&get_todo_file())
}

fn run_command(args: &[String], todo_list: &mut TodoList) -> Result<(), TodoError> {
    let command: &str = args.get(1).map(String::as_str).unwrap_or("list");
    match command {
        "l" | "list" => {
            commands::list::list_items(todo_list);
        }
        "a" | "add" | "append" => {
            commands::add::add_item(todo_list, args)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "r" | "rm" | "remove" => {
            commands::remove::remove_item_index(todo_list, args)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "c" | "check" | "complete" => {
            commands::complete::complete_item_index(todo_list, args)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "u" | "un" | "uncheck" | "uncomplete" => {
            commands::complete::uncomplete_item_index(todo_list, args)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        _ => {
            eprintln!("Unrecognized command: '{}'", command);
            std::process::exit(1);
        }
    }
    Ok(())
}

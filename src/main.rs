mod commands;
mod data;
mod prompts;
mod todo_error;
use std::{env, path::PathBuf};

use data::todo_list::TodoList;
use todo_error::TodoError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = run_todo_cli(&args) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}

fn run_todo_cli(args: &[String]) -> Result<(), TodoError> {
    let mut todo_list = get_todo_list()?;
    run_command(args, &mut todo_list)?;
    Ok(())
}

fn get_todo_list() -> Result<TodoList, TodoError> {
    let mut todo_file = get_todo_file();

    if !todo_file.exists() {
        let local = PathBuf::from(".todo");
        let create_new_todo = prompts::prompt_create_new_todo_list();
        if !create_new_todo {
            return Err(TodoError::Aborted);
        }
        std::fs::File::create_new(&local)?;
        todo_file = local;
    }

    TodoList::from_file(&todo_file)
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
            let completed = false;
            commands::add::add_item(todo_list, args, completed)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "as" | "adds" | "appends" => {
            let completed = false;
            commands::add::add_several(todo_list, args, completed)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "log" => {
            let completed = true;
            commands::add::add_item(todo_list, args, completed)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "logs" => {
            let completed = true;
            commands::add::add_several(todo_list, args, completed)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "r" | "rm" | "remove" => {
            commands::remove::remove_item(todo_list, args)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "c" | "check" | "complete" => {
            commands::complete::complete_item(todo_list, args)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "u" | "un" | "uncheck" | "uncomplete" => {
            commands::complete::uncomplete_item(todo_list, args)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "clean" => {
            commands::clean::clean_items(todo_list, args)?;
            save_current_todo_list(todo_list)?;
            commands::list::list_items(todo_list);
        }
        "help" => {
            commands::help::print_help();
        }
        _ => return Err(TodoError::BadCommand(command.to_string())),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    fn basic_todo_list() -> TodoList {
        let list = [
            "- [ ] Wash dishes",
            "- [ ] Clean clothes",
            "- [x] Take out trash the trash",
            "- [ ] Laundry",
        ];
        let todo_list: TodoList = list.join("\n").parse().unwrap();
        todo_list
    }

    #[test]
    fn test_add() {
        let mut todo_list: TodoList = TodoList::default();
        let args = vec_of_strings!["todo", "add", "first!"];
        let result = run_command(&args, &mut todo_list);
        assert!(result.is_ok());
        assert!(todo_list.get_items_len() == 1);
    }

    #[test]
    fn test_adds() {
        let mut todo_list: TodoList = TodoList::default();
        let args = vec_of_strings!["todo", "adds", "first!", "second", "third"];
        let result = run_command(&args, &mut todo_list);
        assert!(result.is_ok());
        assert!(todo_list.get_items_len() == 3);
    }

    #[test]
    fn test_complete_single() {
        let mut todo_list = basic_todo_list();
        let args = vec_of_strings!["todo", "complete", "1"];
        assert!(todo_list.get_number_completed() == 1);
        let result = run_command(&args, &mut todo_list);
        assert!(result.is_ok());
        assert!(todo_list.get_items_len() == 4);
        assert!(todo_list.get_number_completed() == 2);
    }

    #[test]
    fn test_complete_range() {
        let mut todo_list = basic_todo_list();
        let args = vec_of_strings!["todo", "complete", "1-2"];
        assert!(todo_list.get_number_completed() == 1);
        let result = run_command(&args, &mut todo_list);
        assert!(result.is_ok());
        assert!(todo_list.get_items_len() == 4);
        assert!(todo_list.get_number_completed() == 3);
    }

    #[test]
    fn test_complete_list() {
        let mut todo_list = basic_todo_list();
        let args = vec_of_strings!["todo", "complete", "1,4"];
        assert!(todo_list.get_number_completed() == 1);
        let result = run_command(&args, &mut todo_list);
        assert!(result.is_ok());
        assert!(todo_list.get_items_len() == 4);
        assert!(todo_list.get_number_completed() == 3);
    }
}

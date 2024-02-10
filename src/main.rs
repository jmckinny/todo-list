use std::path::Path;

mod commands;
mod todo_list;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).unwrap_or(&String::from("list")).to_lowercase();
    let todo_file_path = Path::new(".todo");
    let list = todo_list::TodoList::from_file(todo_file_path).expect("Failed to load todolist");
    match command.as_str() {
        "list" => commands::list::list_command(&list),
        "add" => todo!(),
        "remove" => todo!(),
        _ => {
            eprintln!("No command '{}' found", command)
        }
    }
}

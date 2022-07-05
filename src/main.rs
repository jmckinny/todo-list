use std::{
    env::{self, VarError},
    path::Path,
    process::exit,
};
mod todo_list;
fn main() {
    
    let path = setup_path();
    let working_path = path.as_str();

    let args: Vec<String> = env::args().collect();
    let action = args.get(1);
    let mut list = todo_list::TodoList::from_file(working_path);

    match action {
        None => {
            print!("{}", list);
            return;
        }
        Some(x) => match x.to_lowercase().as_str() {
            "add" | "a" => {
                list.add_item(args.get(2).expect("No item to add!"));
            }
            "rm" | "r" => {
                let index: usize = get_arg_index(&args);
                list.remove_item(index - 1);
            }
            "complete" | "c" => {
                let index: usize = get_arg_index(&args);
                list.complete_item(index - 1);
            }
            "new" | "n" => {
                if Path::new(".todo").exists() {
                    println!("Failure: A todo list already exists here");
                } else {
                    std::fs::File::create(".todo").expect("Failed to create new todo file");
                    println!(
                        "Created new .todo at {:?}",
                        std::env::current_dir().expect("Failed to get current directory")
                    )
                }
            }
            "edit" | "e" => {
                let index: usize = get_arg_index(&args);
                let new_text = args.get(3).expect("Invalid new_text");
                list.edit_item(index - 1, new_text);
            }
            _ => {
                println!("Invalid action provided")
            }
        },
    }
    list.write_file(working_path);
}

fn setup_path() -> String{
    let path = get_path();
    if path.is_err() {
        println!("GLOBAL_TODO_PATH variable not set!\nTry adding 'export GLOBAL_TODO_PATH=/home/$USER/.todo' to your shell");
        exit(-1);
    }
    let path = path.expect("GLOBAL_TODO_PATH not set");
    if !Path::new(path.as_str()).exists() {
        std::fs::File::create(path.as_str()).expect("Failed to intialize todo list");
    }
    path
}

fn get_path() -> Result<String, VarError> {
    if Path::new(".todo").exists() {
        Ok(".todo".to_string())
    } else {
        env::var("GLOBAL_TODO_PATH")
    }
}

fn get_arg_index(args: &[String]) -> usize {
    args.get(2)
        .expect("Invalid index")
        .parse()
        .expect("Failed to convert index")
}

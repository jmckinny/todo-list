use std::{
    env::{self, VarError},
    path::Path,
};
mod todo_list;
fn main() {
    let path = setup_path().expect("GLOBAL_TODO_PATH not set!");
    let working_path = path.as_str();

    let args: Vec<String> = env::args().collect();
    let action = args.get(1);
    let mut list = todo_list::TodoList::from_file(working_path);

    match action {
        None => {
            print!("{}", list)
        }
        Some(x) => match x.to_lowercase().as_str() {
            "add" => {
                list.add_item(args.get(2).expect("No item to add!"));
                list.write_file(working_path);
            }
            "rm" => {
                let index: usize = args
                    .get(2)
                    .expect("Invalid index")
                    .parse()
                    .expect("Failed to convert index");
                list.remove_item(index - 1);
                list.write_file(working_path);
            }
            "new" => {
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
            _ => {
                println!("Invalid action provided")
            }
        },
    }
}

fn setup_path() -> Result<String, VarError> {
    if Path::new(".todo").exists() {
        Ok(".todo".to_string())
    } else {
        env::var("GLOBAL_TODO_PATH")
    }
}

use crate::{todo_error::TodoError, todo_list::TodoList};

pub fn remove_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    if let Some(index) = args.get(2) {
        todo_list.remove_item(index.parse()?)
    } else {
        eprintln!("Usage: todo remove <index>");
        std::process::exit(1);
    }
}

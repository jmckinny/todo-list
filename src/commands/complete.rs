use crate::data::todo_list::TodoList;
use crate::TodoError;

pub fn complete_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    if let Some(index) = args.get(2) {
        todo_list.complete_item(index.parse()?)?;
        Ok(())
    } else {
        eprintln!("Usage: todo complete <index>");
        std::process::exit(1);
    }
}

pub fn uncomplete_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    if let Some(index) = args.get(2) {
        todo_list.uncomplete_item(index.parse()?)?;
        Ok(())
    } else {
        eprintln!("Usage: todo uncomplete <index>");
        std::process::exit(1);
    }
}

use crate::data::todo_list::TodoList;
use crate::get_todo_file;
use crate::TodoError;

pub fn remove_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    if let Some(index) = args.get(2) {
        todo_list.remove_item(index.parse()?)?;
        todo_list.save_to_file(&get_todo_file())?;
        Ok(())
    } else {
        eprintln!("Usage: todo remove <index>");
        std::process::exit(1);
    }
}

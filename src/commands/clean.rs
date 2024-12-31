use crate::{data::todo_list::TodoList, todo_error::TodoError};

pub fn clean_items(todo_list: &mut TodoList, _: &[String]) -> Result<(), TodoError> {
    todo_list.remove_completed_items();
    Ok(())
}

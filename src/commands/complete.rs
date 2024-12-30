use crate::data::todo_list::TodoList;
use crate::TodoError;

use super::utils::get_index;

pub fn complete_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index = get_index(args)?;
    todo_list.complete_item(index)?;
    Ok(())
}

pub fn uncomplete_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index = get_index(args)?;
    todo_list.uncomplete_item(index)?;
    Ok(())
}

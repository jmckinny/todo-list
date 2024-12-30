use super::index_utils::get_index;
use crate::data::todo_list::TodoList;
use crate::TodoError;

pub fn remove_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index = get_index(args)?;
    todo_list.remove_item(index)?;
    Ok(())
}

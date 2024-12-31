use super::index_utils::get_index;
use super::index_utils::IndexSelection;
use crate::data::todo_list::TodoList;
use crate::TodoError;

pub fn remove_item(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index_selection = get_index(todo_list, args)?;
    match index_selection {
        IndexSelection::Single(index) => {
            todo_list.remove_item(index)?;
        }
        IndexSelection::Range((start, end)) => {
            todo_list.remove_items(start, end)?;
        }
    }
    Ok(())
}

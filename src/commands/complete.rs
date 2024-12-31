use super::index_utils::get_index;
use super::index_utils::IndexSelection;
use crate::data::todo_list::TodoList;
use crate::TodoError;

pub fn complete_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index_selection = get_index(args)?;
    match index_selection {
        IndexSelection::Single(index) => {
            todo_list.set_item_completion(index, true)?;
        }
        IndexSelection::Range((start, end)) => {
            todo_list.complete_items(start, end)?;
        }
    }
    Ok(())
}

pub fn uncomplete_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index_selection = get_index(args)?;
    match index_selection {
        IndexSelection::Single(index) => {
            todo_list.set_item_completion(index, false)?;
        }
        IndexSelection::Range((start, end)) => {
            todo_list.uncomplete_items(start, end)?;
        }
    }
    Ok(())
}

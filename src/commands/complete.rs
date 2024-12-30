use super::index_utils::get_index;
use super::index_utils::IndexSelection;
use crate::data::todo_list::TodoList;
use crate::TodoError;

pub fn complete_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index_selection = get_index(args)?;
    match index_selection {
        IndexSelection::Single(index) => {
            todo_list.complete_item(index)?;
        }
        IndexSelection::Range((start, end)) => {
            todo!()
        }
    }
    Ok(())
}

pub fn uncomplete_item_index(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index_selection = get_index(args)?;
    match index_selection {
        IndexSelection::Single(index) => {
            todo_list.uncomplete_item(index)?;
        }
        IndexSelection::Range((start, end)) => {
            todo!()
        }
    }
    Ok(())
}

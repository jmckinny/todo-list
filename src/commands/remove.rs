use super::index_utils::get_index;
use super::index_utils::IndexSelection;
use crate::data::todo_list::TodoList;
use crate::prompts::prompt_remove_item;
use crate::TodoError;

pub fn remove_item(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let index_selection = get_index(todo_list, args)?;
    match index_selection {
        IndexSelection::Single(index) => {
            if prompt_remove_item(1) {
                todo_list.remove_item(index)?;
            }
        }
        IndexSelection::Range((start, end)) => {
            if prompt_remove_item((end - start) + 1) {
                todo_list.remove_items(start, end)?;
            }
        }
        IndexSelection::List(mut lst) => {
            if prompt_remove_item(lst.len()) {
                lst.sort();
                for index in lst.into_iter().rev() {
                    todo_list.remove_item(index)?;
                }
            }
        }
    }
    Ok(())
}

use crate::data::todo_item::TodoItem;
use crate::data::todo_list::TodoList;
use crate::get_todo_file;
use crate::todo_error::TodoError;

pub fn add_item(todo_list: &mut TodoList, args: &[String]) -> Result<(), TodoError> {
    let content = args
        .iter()
        .skip(2)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    let completed = false;
    let item = TodoItem::new(content, completed);
    todo_list.add_item(item);
    todo_list.save_to_file(&get_todo_file())?;
    Ok(())
}

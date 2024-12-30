use crate::{
    todo_error::TodoError,
    todo_list::{TodoItem, TodoList},
};

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
    Ok(())
}

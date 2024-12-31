use crate::data::todo_item::TodoItem;
use crate::data::todo_list::TodoList;
use crate::todo_error::TodoError;

pub fn add_item(
    todo_list: &mut TodoList,
    args: &[String],
    completed: bool,
) -> Result<(), TodoError> {
    let content = args
        .iter()
        .skip(2)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    let item = TodoItem::new(content, completed);
    todo_list.add_item(item);
    Ok(())
}

pub fn add_several(
    todo_list: &mut TodoList,
    args: &[String],
    completed: bool,
) -> Result<(), TodoError> {
    let content = args.iter().skip(2).map(String::as_str).collect::<Vec<_>>();
    for item_content in content {
        let item = TodoItem::new(item_content.to_string(), completed);
        todo_list.add_item(item);
    }
    Ok(())
}

use crate::data::todo_item::TodoItem;
use std::path::Path;
use std::str::FromStr;

use crate::todo_error::TodoError;

#[derive(Debug, Default)]
pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn from_file(path: &Path) -> Result<Self, TodoError> {
        let file_data = std::fs::read_to_string(path)?;
        file_data.parse()
    }

    pub fn remove_item(&mut self, index: usize) -> Result<(), TodoError> {
        if index > self.items.len() {
            return Err(TodoError::InvalidItem(format!(
                "Item index '{}' does not exist",
                index
            )));
        }
        self.items.remove(index - 1);
        Ok(())
    }

    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item);
    }
}

impl FromStr for TodoList {
    type Err = TodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item_results: Result<Vec<TodoItem>, _> = s.lines().map(|line| line.parse()).collect();
        let items = item_results?;
        Ok(TodoList { items })
    }
}

impl std::fmt::Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.items.is_empty() {
            return writeln!(f, "Nothing todo!");
        }

        for (i, item) in self.items.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, item)?;
        }

        Ok(())
    }
}

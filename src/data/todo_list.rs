use crate::data::todo_item::TodoItem;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::str::FromStr;

use crate::todo_error::TodoError;

use super::todo_item::{COMPLETED_PREFIX, UNCOMPLETED_PREFIX};

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
        if self.invalid_index(index) {
            return Err(TodoError::InvalidItem(format!(
                "Item index '{}' does not exist",
                index
            )));
        }
        self.items.remove(index - 1);
        Ok(())
    }

    pub fn remove_items(&mut self, start: usize, end: usize) -> Result<(), TodoError> {
        if self.invalid_index(start) || self.invalid_index(end) {
            return Err(TodoError::InvalidItem(format!(
                "Item indices '{}-{}' do not exist",
                start, end
            )));
        }
        self.items.drain(start - 1..=end - 1);
        Ok(())
    }

    pub fn complete_items(&mut self, start: usize, end: usize) -> Result<(), TodoError> {
        if self.invalid_index(start) || self.invalid_index(end) {
            return Err(TodoError::InvalidItem(format!(
                "Item indices '{}-{}' do not exist",
                start, end
            )));
        }
        for (i, item) in self.items.iter_mut().enumerate() {
            if (start - 1..=end - 1).contains(&i) {
                item.complete_item();
            }
        }
        Ok(())
    }

    pub fn uncomplete_items(&mut self, start: usize, end: usize) -> Result<(), TodoError> {
        if self.invalid_index(start) || self.invalid_index(end) {
            return Err(TodoError::InvalidItem(format!(
                "Item indices '{}-{}' do not exist",
                start, end
            )));
        }
        for (i, item) in self.items.iter_mut().enumerate() {
            if (start - 1..=end - 1).contains(&i) {
                item.uncomplete_item();
            }
        }
        Ok(())
    }

    pub fn set_item_completion(&mut self, index: usize, complete: bool) -> Result<(), TodoError> {
        if self.invalid_index(index) {
            return Err(TodoError::InvalidItem(format!(
                "Item index '{}' does not exist",
                index
            )));
        }
        if complete {
            self.items[index - 1].complete_item();
        } else {
            self.items[index - 1].uncomplete_item();
        }
        Ok(())
    }

    pub fn remove_completed_items(&mut self) {
        self.items.retain(|item| !item.is_complete());
    }

    pub fn get_items(&self) -> &Vec<TodoItem> {
        &self.items
    }

    pub fn get_items_len(&self) -> usize {
        self.items.len()
    }

    pub fn get_number_completed(&self) -> usize {
        self.items.iter().filter(|item| item.is_complete()).count()
    }

    fn invalid_index(&self, index: usize) -> bool {
        index > self.items.len()
    }

    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item);
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        for item in self.items.iter() {
            if item.is_complete() {
                write!(writer, "{} ", COMPLETED_PREFIX)?;
            } else {
                write!(writer, "{} ", UNCOMPLETED_PREFIX)?;
            }
            writeln!(writer, "{}", item.get_content())?;
        }
        writer.flush()
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

use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

#[derive(Debug)]
pub struct TodoList {
    list: Vec<String>,
}

impl TodoList {
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let mut list = Vec::new();
        let reader = BufReader::new(File::open(path).expect("TODO list file not found"));
        for line in reader.lines() {
            list.push(line.expect("Failed to read TODO list line"));
        }
        TodoList { list }
    }

    pub fn write_file(&self, path: impl AsRef<Path>) {
        let mut file = File::create(path).expect("Unable to create file!");
        for item in &self.list {
            writeln!(file, "{}", item).expect("Failed to write line to file!");
        }
    }

    pub fn add_item(&mut self, to_add: &str) {
        self.list.push(to_add.to_string());
        println!("Added: {}", to_add);
    }

    pub fn remove_item(&mut self, index: usize) {
        println!("Removed: {}", self.list.get(index).expect("Invalid index"));
        self.list.remove(index);
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn complete_item(&mut self, index: usize) {
        let item = self.list.get(index).expect("Invalid index");
        self.list[index] = item
            .chars()
            .map(|f| "\u{0336}".to_owned() + &f.to_string() + "\u{0336}")
            .collect();
        println!("Completed: {}", self.list.get(index).unwrap());
    }

    pub fn edit_item(&mut self, index: usize, new_text: &str) {
        self.list[index] = new_text.to_string();
        println!("Updated: {}. {}", index + 1, new_text);
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            writeln!(f, "Nothing todo!")?;
        }
        for (i, item) in self.list.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, item)?;
        }
        Ok(())
    }
}

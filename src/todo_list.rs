use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        let file_data = std::fs::read_to_string(path)?;
        file_data.parse()
    }
}

#[derive(Debug)]
struct TodoItem {
    content: String,
}

impl FromStr for TodoList {
    type Err = std::io::Error;

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

impl FromStr for TodoItem {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TodoItem {
            content: s.to_owned(),
        })
    }
}

impl std::fmt::Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

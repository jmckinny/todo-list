use std::{path::Path, str::FromStr};

pub struct TodoList {
    items: Vec<String>,
}

#[derive(Debug)]
pub enum TodoListParseError {
    FileError(std::io::Error),
    ParseError,
}

impl From<std::io::Error> for TodoListParseError {
    fn from(value: std::io::Error) -> Self {
        Self::FileError(value)
    }
}

impl FromStr for TodoList {
    // TODO: Better error checking
    type Err = TodoListParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TodoList {
            items: s.lines().map(|s| s.to_owned()).collect(),
        })
    }
}

impl TodoList {
    pub fn from_file(path: &Path) -> Result<Self, TodoListParseError> {
        let string = std::fs::read_to_string(path)?;
        string.parse()
    }
}

impl std::fmt::Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, item) in self.items.iter().enumerate() {
            writeln!(f, "{}. {}", index + 1, item)?;
        }
        Ok(())
    }
}

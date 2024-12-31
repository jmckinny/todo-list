use std::str::FromStr;

use crate::TodoError;
pub const COMPLETED_PREFIX: &str = "- [x]";
pub const UNCOMPLETED_PREFIX: &str = "- [ ]";

#[derive(Debug)]
pub struct TodoItem {
    content: String,
    completed: bool,
}

impl TodoItem {
    pub fn new(content: String, completed: bool) -> Self {
        Self { content, completed }
    }

    pub fn is_complete(&self) -> bool {
        self.completed
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn complete_item(&mut self) {
        self.completed = true;
    }

    pub fn uncomplete_item(&mut self) {
        self.completed = false;
    }
}

impl FromStr for TodoItem {
    type Err = TodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(s.starts_with(UNCOMPLETED_PREFIX) || s.starts_with(COMPLETED_PREFIX)) {
            return Err(TodoError::ParseError(format!(
                "Failed to parse item: '{}' ",
                s
            )));
        }

        let completed = s.starts_with(COMPLETED_PREFIX);
        let content = if completed {
            s.strip_prefix(COMPLETED_PREFIX)
                .unwrap()
                .trim_start()
                .to_owned()
        } else {
            s.strip_prefix(UNCOMPLETED_PREFIX)
                .unwrap()
                .trim_start()
                .to_owned()
        };

        Ok(TodoItem { content, completed })
    }
}

impl std::fmt::Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.completed {
            write!(f, "[x] ")?;
        } else {
            write!(f, "[ ] ")?;
        }
        write!(f, "{}", self.content)
    }
}

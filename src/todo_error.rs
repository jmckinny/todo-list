use std::num::ParseIntError;

#[derive(Debug)]
pub enum TodoError {
    IoError(std::io::Error),
    ParseError(String),
    InvalidItem(String),
    ParseIndexError(ParseIntError),
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::IoError(error) => writeln!(f, "IO Error: {}", error),
            TodoError::ParseError(s) => writeln!(f, "Parse Error: {}", s),
            TodoError::InvalidItem(s) => writeln!(f, "Invalid item: {}", s),
            TodoError::ParseIndexError(s) => writeln!(f, "Invalid index: {}", s),
        }
    }
}

impl std::error::Error for TodoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            TodoError::IoError(ref error) => Some(error),
            TodoError::ParseIndexError(ref error) => Some(error),
            TodoError::ParseError(_) => None,
            TodoError::InvalidItem(_) => None,
        }
    }
}

impl From<std::io::Error> for TodoError {
    fn from(value: std::io::Error) -> Self {
        TodoError::IoError(value)
    }
}

impl From<ParseIntError> for TodoError {
    fn from(value: ParseIntError) -> Self {
        TodoError::ParseIndexError(value)
    }
}

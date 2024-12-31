use std::num::ParseIntError;

#[derive(Debug)]
pub enum TodoError {
    IoError(std::io::Error),
    ParseError(String),
    InvalidItem(String),
    ParseIndexError(ParseIntError),
    BadCommand(String),
    UsageError(String),
    FuzzyMatchFailed(String),
    Aborted,
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::IoError(error) => writeln!(f, "IO Error: {}", error),
            TodoError::ParseError(s) => writeln!(f, "Parse Error: {}", s),
            TodoError::InvalidItem(s) => writeln!(f, "Invalid item: {}", s),
            TodoError::ParseIndexError(s) => writeln!(f, "Invalid index: {}", s),
            TodoError::BadCommand(command) => writeln!(f, "Invalid command: {}", command),
            TodoError::UsageError(usage) => writeln!(f, "{}", usage),
            TodoError::FuzzyMatchFailed(serach) => writeln!(f, "No item matched: '{}'", serach),
            TodoError::Aborted => writeln!(f, "Aborting"),
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
            TodoError::BadCommand(_) => None,
            TodoError::UsageError(_) => None,
            TodoError::FuzzyMatchFailed(_) => None,
            TodoError::Aborted => None,
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

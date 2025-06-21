use std::fmt;

#[derive(Debug)]
pub enum StatError {
    InvalidStats(String),
    PlayerNotFound(String),
    IoError(String),
}

impl fmt::Display for StatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatError::InvalidStats(msg) => write!(f, "Invalid Stats: {}", msg),
            StatError::PlayerNotFound(msg) => write!(f, "Player Not Found: {}", msg),
            StatError::IoError(msg) => write!(f, "I/O Error: {}", msg),
        }
    }
}

impl From<std::io::Error> for StatError {
    fn from(error: std::io::Error) -> Self {
        StatError::IoError(error.to_string())
    }
}

impl From<serde_json::Error> for StatError {
    fn from(error: serde_json::Error) -> Self {
        StatError::IoError(error.to_string())
    }
}
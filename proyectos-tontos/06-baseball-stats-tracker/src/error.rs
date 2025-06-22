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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_invalid_stats_error_display() {
        let error = StatError::InvalidStats("At bats cannot be negative".to_string());
        assert_eq!(format!("{}", error), "Invalid Stats: At bats cannot be negative");
    }

    #[test]
    fn test_player_not_found_error_display() {
        let error = StatError::PlayerNotFound("Mick Letofsky".to_string());
        assert_eq!(format!("{}", error), "Player Not Found: Mick Letofsky");
    }

    #[test]
    fn test_io_error_display() {
        let error = StatError::IoError("Failed to read file".to_string());
        assert_eq!(format!("{}", error), "I/O Error: Failed to read file");
    }

    #[test]
    fn test_from_io_error_conversion() {
        // Create a real io::Error
        let io_error = std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            "players.json not found"
        );
        
        // Convert it to StatError using From trait
        let stat_error: StatError = io_error.into();
        
        // Verify it's the right variant
        match stat_error {
            StatError::IoError(msg) => {
                assert!(msg.contains("players.json"));
                assert!(msg.contains("not found"));
            },
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_from_serde_json_error_conversion() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json");
        let stat_error: StatError = json_err.err().unwrap().into();
        match stat_error {
            StatError::IoError(msg) => {
                assert!(msg.contains("expected value"));
            },
            _ => panic!("Expected IoError variant"),
        }
    }

}
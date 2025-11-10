//! Golf Score Tracker Error Types
//! 
//! This module demonstrates Rust's error handling patterns:
//! - Custom error types with `thiserror`
//! - Error composition and context
//! - Converting between error types

use thiserror::Error;

/// Primary golf score tracker app error type
#[derive(Error, Debug)]
pub enum GolfError {

    /// Invalid score for a hole
    /// 
    /// This variant carries three pieces of data context:
    /// - {0} = the invalid score
    /// - {1} = the hole number
    /// - {2} = the par for that hole
    #[error("Invalid score {score} for hole {hole} (par {par}). Score must be between 1 and 15.")]
    InvalidScore { score: i32, hole: u8, par: u8 },

    /// Hole number out of range
    #[error("Hole number {hole} is invalid. Must be between 1 and {max_holes}.")]
    InvalidHole { hole: u8, max_holes: u8 },

    /// Player not found
    #[error("Player '{0}' not found")]
    PlayerNotFound(String),

    /// Round not found
    #[error("Round with ID {0} not found")]
    RoundNotFound(uuid::Uuid),

    /// Invalid par value for a hole
    #[error("Par {0} is invalid. Must be 3, 4, or 5.")]
    InvalidPar(u8),

    /// Scorecard already completed
    #[error("Scorecard for round {0} is already complete")]
    ScorecardComplete(uuid::Uuid),

    /// The `#[from]` attribute automatically implements conversion
    /// This allows the `?` operator to work seamlessly
    #[error("Failed to serialize/deserialize data")]
    SerializationError(#[from] serde_json::Error),

    /// IO Error wrapper
    #[error("File operation failed")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    Custom(String),
}

impl GolfError {
    pub fn custom(msg: impl Into<String>) -> Self {
        GolfError::Custom(msg.into())
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self, GolfError::PlayerNotFound(_) | GolfError::RoundNotFound(_))
    }

    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            GolfError::InvalidScore { .. } | GolfError::InvalidHole { .. } | GolfError::InvalidPar(_)
        )
    }
}

pub type Result<T> = std::result::Result<T, GolfError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_score_error() {
        let error = GolfError::InvalidScore{score: 20, hole: 5, par: 4};
        let result = error.to_string();

        assert_eq!(
            result,
            "Invalid score 20 for hole 5 (par 4). Score must be between 1 and 15."
        );
    }

    #[test]
    fn invalid_hole_error_message() {
        let error = GolfError::InvalidHole { hole: 19,max_holes: 9};
        let result = error.to_string();

        assert_eq!(
            result,
            "Hole number 19 is invalid. Must be between 1 and 9."
        );
    }

    #[test]
    fn error_classification() {
        let not_found = GolfError::PlayerNotFound("Alice Bob Smith".to_string());
        assert!(not_found.is_not_found());
        assert!(!not_found.is_validation_error());

        let validation = GolfError::InvalidPar(7);
        assert!(validation.is_validation_error());
        assert!(!validation.is_not_found());
    }

    #[test]
    fn error_conversion_from_io() {
        fn read() -> Result<String>{
            let data = std::fs::read_to_string("non_existent_file.txt")?;
            Ok(data)
        }
        let result = read();
        assert!(result.is_err());
        match result {
            Err(GolfError::IoError(_)) => {},
            _ => panic!("Expected IoError variant"),
        }
    }
}
//! Error types and result aliases for the golf score tracker.
//!
//! This module defines a custom error type using the `thiserror` crate,
//! providing structured error handling throughout the application.
//!
//! # Examples
//!
//! ```
//! use golf_score_tracker::{GolfError, Result};
//!
//! fn validate_score(score: i32, hole: u8, par: u8) -> Result<()> {
//!     if score < 1 || score > 15 {
//!         return Err(GolfError::InvalidScore { score, hole, par });
//!     }
//!     Ok(())
//! }
//!
//! assert!(validate_score(4, 1, 4).is_ok());
//! assert!(validate_score(0, 1, 4).is_err());
//! ```
use thiserror::Error;

/// Result type alias for golf tracker operations.
///
/// This type alias simplifies function signatures by defaulting the error
/// type to `GolfError`. Use this for all operations that may fail within
/// the golf tracker domain.
///
pub type Result<T> = std::result::Result<T, GolfError>;


/// Errors that can occur during golf score tracking operations.
///
/// This enum represents all domain-specific errors in the application.
/// Each variant includes context to help diagnose and handle errors
/// appropriately.
///
/// # Variants
///
/// * `InvalidHole` - Hole number outside valid range (1 to max_holes)
/// * `InvalidScore` - Score value unrealistic (0 or > 15)
/// * `InvalidPar` - Par value outside valid range (3-5)
/// * `ScorecardComplete` - Attempted to modify completed scorecard
/// * `Io` - File system or I/O operation failed
/// * `SerdeJson` - JSON serialization/deserialization failed
///
/// # Examples
///
/// ```
/// use golf_score_tracker::GolfError;
///
/// let error = GolfError::InvalidScore { score: 20, hole: 5, par: 4 };
/// assert_eq!(error.to_string(), "Invalid score 20 for hole 5 (par 4). Score must be between 1 and 15.");
/// ```
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

    /// Hole number is outside the valid range for the scorecard.
    ///
    /// # Examples
    ///
    /// ```
    /// use golf_score_tracker::GolfError;
    ///
    /// let error = GolfError::InvalidHole { hole: 19, max_holes: 18 };
    /// assert!(error.to_string().contains("19"));
    /// ```
    #[error("Hole number {hole} is invalid. Must be between 1 and {max_holes}.")]
    InvalidHole { hole: u8, max_holes: u8 },

    /// Player not found
    #[error("Player '{0}' not found")]
    PlayerNotFound(String),

    /// Round not found
    #[error("Round with ID {0} not found")]
    RoundNotFound(uuid::Uuid),

    /// Par value is outside the standard golf range.
    ///
    /// Standard golf holes have par values of 3, 4, or 5. Other values
    /// are non-standard and rejected.
    ///
    /// # Examples
    ///
    /// ```
    /// use golf_score_tracker::GolfError;
    ///
    /// let error = GolfError::InvalidPar(6);
    /// assert!(error.to_string().contains("6"));
    /// ```
    #[error("Par {0} is invalid. Must be 3, 4, or 5.")]
    InvalidPar(u8),

    /// Attempted to modify a scorecard that is already complete.
    ///
    /// Once all holes are recorded, the scorecard is considered complete
    /// and immutable. This prevents accidental data corruption.
    #[error("Scorecard for round {0} is already complete")]
    ScorecardComplete(uuid::Uuid),

    /// The `#[from]` attribute automatically implements conversion
    /// This allows the `?` operator to work seamlessly
    #[error("Failed to serialize/deserialize data")]
    SerializationError(#[from] serde_json::Error),

    /// File system or I/O operation failed.
    ///
    /// This wraps standard library I/O errors that occur during repository
    /// operations (reading/writing scorecards, creating directories, etc.).
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
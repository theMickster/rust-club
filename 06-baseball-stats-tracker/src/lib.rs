//! Baseball Statistics Tracker
//! 
//! A library for tracking and analyzing baseball player statistics.

// Declare modules
mod error;
mod stats;
mod player;
mod tracker;

// Re-export public API
pub use error::StatError;
pub use stats::BattingStats;
pub use player::Player;
pub use tracker::StatsTracker;
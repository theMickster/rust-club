use serde::{Deserialize, Serialize};
use std::fmt;

use crate::BattingStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub team: String,
    pub position: String,
    pub batting_stats: BattingStats,
}

impl Player {
    pub fn new(name: String, team: String, position:String) -> Self {
        Player {
            name,
            team,
            position,
            batting_stats: BattingStats::new(),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) - {} | {}",
            self.name,
            self.team,
            self.position,
            self.batting_stats
        )
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.batting_stats.ops() == other.batting_stats.ops() 
    }
}

impl Eq for Player {}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.batting_stats.ops().partial_cmp(&other.batting_stats.ops())
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

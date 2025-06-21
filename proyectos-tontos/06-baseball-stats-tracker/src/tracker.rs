use serde::{Deserialize, Serialize};

use crate::{BattingStats, Player, StatError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsTracker {
    players: Vec<Player>,
}

impl StatsTracker {
    pub fn new() -> Self {
        StatsTracker {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) -> Result<(), StatError> {
        if self.players.iter().any(|p| p.name == player.name && p.team == player.team) {
            return Err(StatError::InvalidStats(
                format!("Player {} already exists on {}", player.name, player.team)
            ));
        }
        self.players.push(player);
        Ok(())
    }

    pub fn find_player(&self, name: &str) -> Result<&Player, StatError> {
        self.players
            .iter()
            .find(|p| p.name == name)
            .ok_or_else(|| StatError::PlayerNotFound(format!("Player {} not found", name)))
    }

    pub fn find_player_mut(&mut self, name: &str) -> Result<&mut Player, StatError> {
        self.players
            .iter_mut()
            .find(|p| p.name == name)
            .ok_or_else(|| StatError::PlayerNotFound(format!("Player {} not found", name)))
    }

    pub fn update_player(&mut self, name: &str, stats: BattingStats) -> Result<(), StatError> {
        let player = self.find_player_mut(name)?;
        player.batting_stats = stats;
        Ok(())
    }

    pub fn remove_player(&mut self, name: &str) -> Result<Player, StatError> {
        let position = self.players
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| StatError::PlayerNotFound(format!("Player '{}' not found", name)))?;
        
        Ok(self.players.remove(position))
    }

    pub fn get_players(&self) -> &[Player] {
        &self.players
    }

    pub fn leaderboard_by_ops(&self) -> Vec<Player> {
        let mut sorted = self.players.clone();
        sorted.sort_by(|a, b| b.cmp(a)); 
        sorted
    }
    
    pub fn leaderboard_by_home_runs(&self) -> Vec<Player> {
        let mut sorted = self.players.clone();
        sorted.sort_by(|a, b| b.batting_stats.home_runs.cmp(&a.batting_stats.home_runs));
        sorted
    }

    pub fn leaderboard_by_avg(&self) -> Vec<Player> {
        let mut sorted = self.players.clone();
        sorted.sort_by(|a, b| {
            b.batting_stats.batting_average()
                .partial_cmp(&a.batting_stats.batting_average())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }

    pub fn count(&self) -> usize {
        self.players.len()
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), StatError> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, StatError> {
        let data = std::fs::read_to_string(path)?;
        let tracker: StatsTracker = serde_json::from_str(&data)?;
        Ok(tracker)
    }

}

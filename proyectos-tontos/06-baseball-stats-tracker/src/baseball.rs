use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum StatError {
    InvalidStats(String),
    PlayerNotFound(String),
    IoError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattingStats {
    pub at_bats: u32,
    pub hits: u32,
    pub singles: u32,
    pub doubles: u32,
    pub triples: u32,
    pub home_runs: u32,
    pub runs_batted_in: u32,
    pub walks: u32,
    pub strikeouts: u32,
}

impl BattingStats {
    pub fn new() -> Self{
        BattingStats {
            at_bats: 0,
            hits: 0,
            singles: 0,
            doubles: 0,
            triples: 0,
            home_runs: 0,
            runs_batted_in: 0,
            walks: 0,
            strikeouts: 0,
        }
    }

    pub fn batting_average(&self) -> f32 {
        if self.at_bats == 0 {
            0.0
        } else {
            self.hits as f32 / self.at_bats as f32
        }
    }

    pub fn slugging_percentage(&self) -> f32 {
        if self.at_bats == 0 {
            0.0
        } else {
            let total_bases = self.singles + (self.doubles * 2) + (self.triples * 3) + (self.home_runs * 4);
            total_bases as f32 / self.at_bats as f32
        }
    }

    pub fn on_base_percentage(&self) -> f32 {
        let plate_appearances = self.at_bats + self.walks;
        if plate_appearances == 0 {
            0.0
        } else {
            (self.hits + self.walks) as f32 / plate_appearances as f32
        }
    }

    pub fn ops(&self) -> f32 {
        self.on_base_percentage() + self.slugging_percentage()
    }
}

impl fmt::Display for BattingStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Batting Stats ==> AVG: {:3} || HR: {} || RBI: {} || OPS: {:.3}",
            self.batting_average(),
            self.home_runs,
            self.runs_batted_in,
            self.ops()
        )
    }
}

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
}
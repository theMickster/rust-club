use std::fmt;
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

impl fmt::Display for StatsTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for player in &self.players {
            writeln!(f, "{}", player)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn get_mickey_mantle() -> Player {
        Player::new("Mickey Mantle".to_string(), "New York Yankees".to_string(), "CF".to_string())
    }

    fn get_lou_gehrig() -> Player {
        Player::new("Lou Gehrig".to_string(), "New York Yankees".to_string(), "1B".to_string())
    }

    fn get_joe_dimaggio() -> Player {
        Player::new("Joe DiMaggio".to_string(), "New York Yankees".to_string(), "CF".to_string())
    }

    fn get_babe_ruth() -> Player {
        Player::new("Babe Ruth".to_string(), "New York Yankees".to_string(), "RF".to_string())
    }

    fn get_hideki_matsui() -> Player {
        Player::new("Hideki Matsui".to_string(), "New York Yankees".to_string(), "LF".to_string())
    }

    fn fixture_players_with_details() -> StatsTracker {
        let mut tracker = StatsTracker::new();
        let mut mm = get_mickey_mantle();
        mm.batting_stats = BattingStats {
            at_bats: 100,
            hits: 40,
            singles: 20,
            doubles: 10,
            triples: 0,
            home_runs: 10,
            runs_batted_in: 30,
            walks: 15,
            strikeouts: 20,
        };
        let mut lg = get_lou_gehrig();
        lg.batting_stats = BattingStats {
            at_bats: 100,
            hits: 30,
            singles: 20,
            doubles: 3,
            triples: 1,
            home_runs: 6,
            runs_batted_in: 15,
            walks: 10,
            strikeouts: 25,
        };
        let mut jd = get_joe_dimaggio();
        jd.batting_stats = BattingStats {
            at_bats: 100,
            hits: 20,
            singles: 20,
            doubles: 0,
            triples: 0,
            home_runs: 0,
            runs_batted_in: 5,
            walks: 5,
            strikeouts: 30,
        };
        let mut br = get_babe_ruth();
        br.batting_stats = BattingStats {
            at_bats: 100,
            hits: 25,
            singles: 15,
            doubles: 5,
            triples: 2,
            home_runs: 3,
            runs_batted_in: 20,
            walks: 10,
            strikeouts: 15,
        };
        let mut hm = get_hideki_matsui();
        hm.batting_stats = BattingStats {
            at_bats: 100,
            hits: 35,
            singles: 25,
            doubles: 5,
            triples: 1,
            home_runs: 4,
            runs_batted_in: 25,
            walks: 12,
            strikeouts: 18,
        };
        
        tracker.add_player(mm).unwrap();
        tracker.add_player(lg).unwrap();
        tracker.add_player(jd).unwrap();
        tracker.add_player(br).unwrap();
        tracker.add_player(hm).unwrap();
        tracker
            }

    #[test]
    fn display() {
        let mut tracker = StatsTracker::new();
        tracker.add_player(get_mickey_mantle()).unwrap();
        tracker.add_player(get_lou_gehrig()).unwrap();
        let output = format!("{}", tracker);
        assert!(output.contains("Mickey Mantle"));
        assert!(output.contains("Lou Gehrig"));
    }

    #[test]
    fn new_tracker_is_empty(){
        let tracker = StatsTracker::new();
        assert_eq!(tracker.count(), 0);
    }

    #[test]
    fn add_player() {
        let mut tracker = StatsTracker::new();
        let player = get_mickey_mantle();
        assert!(tracker.add_player(player).is_ok());
        assert_eq!(tracker.count(), 1);
    }

    #[test]
    fn add_duplicate_player_fails() {        
        let mut tracker = StatsTracker::new();
        let player = get_mickey_mantle();
        tracker.add_player(player).unwrap();

        let duplicate_player = Player::new("Mickey Mantle".to_string(), "New York Yankees".to_string(), "CF".to_string());
        let result = tracker.add_player(duplicate_player);
        assert!(result.is_err());
    }

    #[test]
    fn find_player(){
        let mut tracker = StatsTracker::new();
        let player = get_lou_gehrig();
        tracker.add_player(player.clone()).unwrap();
        
        let found = tracker.find_player("Lou Gehrig");
        let not_found = tracker.find_player("Non Existent");

        assert!(found.is_ok());
        assert_eq!(found.unwrap().name, "Lou Gehrig");
        assert!(not_found.is_err());
    }

    #[test]
    fn find_player_mut(){
        let mut tracker = StatsTracker::new();
        let player = get_hideki_matsui();
        tracker.add_player(player.clone()).unwrap();        
        let result = tracker.find_player_mut("Hideki Matsui");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Hideki Matsui");
        
        let result = tracker.find_player_mut("Non Existent");
        assert!(result.is_err());
    }

    #[test]
    fn get_players(){
        let tracker = fixture_players_with_details();
        let players = tracker.get_players();
        assert_eq!(players.len(), 5);
    }

    #[test]
    fn update_player(){
        let mut tracker = StatsTracker::new();
        let mut player = get_babe_ruth();
        player.batting_stats.hits = 10;
        tracker.add_player(player.clone()).unwrap();

        let mut new_stats = BattingStats::new();
        new_stats.hits = 50;

        let result = tracker.update_player("Babe Ruth", new_stats.clone());
        assert!(result.is_ok());

        let updated_player = tracker.find_player("Babe Ruth").unwrap();
        assert_eq!(updated_player.batting_stats.hits, 50);
    }

    #[test]
    fn remove_player(){
        let mut tracker = StatsTracker::new();
        let player = get_joe_dimaggio();
        tracker.add_player(player.clone()).unwrap();

        let result = tracker.remove_player("Joe DiMaggio");
        assert!(result.is_ok());
        assert_eq!(tracker.count(), 0);

        let result = tracker.remove_player("Non Existent");
        assert!(result.is_err());
    }

    #[test]
    fn leaderboard_by_ops(){
        let tracker = fixture_players_with_details();
        let players = tracker.leaderboard_by_ops();
        assert_eq!(players[0].name, "Mickey Mantle");
        assert_eq!(players[1].name, "Hideki Matsui");
        assert_eq!(players[2].name, "Lou Gehrig");
        assert_eq!(players[3].name, "Babe Ruth");
        assert_eq!(players[4].name, "Joe DiMaggio");
    }

    #[test]
    fn leaderboard_by_home_runs(){
        let tracker = fixture_players_with_details();
        let players = tracker.leaderboard_by_home_runs();
        assert_eq!(players[0].name, "Mickey Mantle");
        assert_eq!(players[1].name, "Lou Gehrig");
        assert_eq!(players[2].name, "Hideki Matsui");
        assert_eq!(players[3].name, "Babe Ruth");
        assert_eq!(players[4].name, "Joe DiMaggio");
    }
    
    #[test]
    fn leaderboard_by_avg(){
        let tracker = fixture_players_with_details();
        let players = tracker.leaderboard_by_avg();
        assert_eq!(players[0].name, "Mickey Mantle");
        assert_eq!(players[1].name, "Hideki Matsui");
        assert_eq!(players[2].name, "Lou Gehrig");
        assert_eq!(players[3].name, "Babe Ruth");
        assert_eq!(players[4].name, "Joe DiMaggio");
    }

}
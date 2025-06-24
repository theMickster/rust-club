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

#[cfg(test)]
mod tests {
    use crate::stats;
    use super::*;

    fn create_player(name: &str, team: &str, position: &str) -> Player {
        Player::new(name.to_string(), team.to_string(), position.to_string())
    }

    fn create_player_with_stats(
        name: &str, 
        team: &str, 
        position: &str,
        at_bats: u32,
        hits: u32,
        singles: u32,
        doubles: u32,
        triples: u32,
        home_runs: u32,
        runs_batted_in: u32,
        walks: u32,
        strikeouts: u32,
    ) -> Player {
        let mut player = create_player(name, team, position);
        player.batting_stats = stats::BattingStats {
            at_bats,
            hits,
            singles,
            doubles,
            triples,
            home_runs,
            runs_batted_in: runs_batted_in,
            walks,
            strikeouts,
        };
        player
    }

    fn fixture_high_ops_player(name: &str) -> Player {
        create_player_with_stats(name, "Test Team", "DH",
            100, 40, 20, 10, 0, 10, 30, 15, 20
        )
    }

    fn fixture_low_ops_player(name: &str) -> Player {
        create_player_with_stats(name, "Test Team", "SS",
            100, 20, 20, 0, 0, 0, 5, 5, 30
        )
    }

    fn fixture_medium_ops_player(name: &str) -> Player {
        create_player_with_stats(name, "Test Team", "CF",
            100, 30, 25, 3, 1, 1, 15, 10, 25
        )
    }

    #[test]
    fn test_player_display(){
        let batting_stats = stats::BattingStats {
            at_bats: 50,
            hits: 14,
            singles: 8,
            doubles: 2,
            triples: 1,
            home_runs: 3,
            runs_batted_in: 5,
            walks: 5,
            strikeouts: 10,   
        };
        let mut player = create_player("Mick Letofsky", "Test Team", "1B");
        player.batting_stats = batting_stats;

        assert_eq!(format!("{}", player), "Mick Letofsky (Test Team) - 1B | Batting Stats ==> AVG: 0.28 || HR: 3 || RBI: 5 || OPS: 0.885");
    }

    #[test]
    fn test_player_new_has_empty_stats(){
        let player = create_player("Pete the Cat", "Test Team", "SS");
        assert_eq!(player.name, "Pete the Cat");
        assert_eq!(player.team, "Test Team");
        assert_eq!(player.position, "SS");
        assert_eq!(player.batting_stats.at_bats, 0);
        assert_eq!(player.batting_stats.hits, 0);
        assert_eq!(player.batting_stats.singles, 0);
        assert_eq!(player.batting_stats.doubles, 0);
        assert_eq!(player.batting_stats.triples, 0);
        assert_eq!(player.batting_stats.home_runs, 0);
        assert_eq!(player.batting_stats.runs_batted_in, 0);
        assert_eq!(player.batting_stats.walks, 0);
        assert_eq!(player.batting_stats.strikeouts, 0);
    }

    #[test]
    fn test_player_equality_same_ops() {
        let player1 = create_player_with_stats(
            "Player A", "Team A", "CF",
            100, 30, 20, 5, 0, 5, 15, 10, 20
        );
        let player2 = create_player_with_stats(
            "Player B", "Team B", "RF",
            1000, 300, 200, 50, 0, 50, 150, 100, 200
        );

        assert_eq!(player1, player2);
    }

    #[test]
    fn test_player_comparison() {
        let high = fixture_high_ops_player("High OPS");
        let low = fixture_low_ops_player("Low OPS");
        
        assert!(high > low);
        assert!(low < high);
    }

    #[test]
    fn test_player_sorting() {
        let mut players = vec![
            fixture_low_ops_player("Low OPS"),
            fixture_high_ops_player("High OPS"),
            fixture_medium_ops_player("Medium OPS"),
        ];
        
        players.sort_by(|a, b| b.cmp(a));
        
        assert!(players[0].name.contains("High"));
        assert!(players[1].name.contains("Medium"));
        assert!(players[2].name.contains("Low"));
    }

}
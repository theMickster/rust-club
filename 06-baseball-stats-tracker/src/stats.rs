use serde::{Deserialize, Serialize};
use std::fmt;

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

#[cfg(test)]
mod tests{
    use super::*;

    fn create_batting_stats(
        at_bats: u32,
        hits: u32,
        singles: u32,
        doubles: u32,
        triples: u32,
        home_runs: u32,
        runs_batted_in: u32,
        walks: u32,
        strikeouts: u32,
    ) -> BattingStats {
        BattingStats {
            at_bats,
            hits,
            singles,
            doubles,
            triples,
            home_runs,
            runs_batted_in,
            walks,
            strikeouts,
        }
    }

    #[test]
    fn test_new_batting_stats_are_empty(){
        let result = BattingStats::new();

        assert_eq!(result.at_bats, 0);
        assert_eq!(result.hits, 0);
        assert_eq!(result.singles, 0);
        assert_eq!(result.doubles, 0);
        assert_eq!(result.triples, 0);
        assert_eq!(result.home_runs, 0);
        assert_eq!(result.runs_batted_in, 0);
        assert_eq!(result.walks, 0);
        assert_eq!(result.strikeouts, 0);
        assert_eq!(result.batting_average(), 0.0);
        assert_eq!(result.slugging_percentage(), 0.0);
        assert_eq!(result.on_base_percentage(), 0.0);
    }

    #[test]
    fn test_batting_stats_display() {
        let result = create_batting_stats(75, 18, 10, 4, 1, 3, 14, 6, 17);
        assert_eq!(format!("{}", result), "Batting Stats ==> AVG: 0.24 || HR: 3 || RBI: 14 || OPS: 0.736");
    }

    #[test]
    fn test_batting_average(){
        let result = create_batting_stats(100, 25, 15, 5, 2, 3, 20, 10, 30);
        assert_eq!(result.batting_average(), 0.25);
    }

    #[test]
    fn test_slugging_percentage(){
        let result = create_batting_stats(100, 25, 15, 5, 2, 3, 20, 10, 30);
        assert_eq!(result.slugging_percentage(), 0.43);
    }

    #[test]
    fn test_on_base_percentage(){
        let result = create_batting_stats(100, 25, 15, 5, 2, 3, 20, 10, 30);
        assert_eq!(result.on_base_percentage(), 0.3181818);
    }

    #[test]
    fn test_ops(){
        let result = create_batting_stats(100, 25, 15, 5, 2, 3, 20, 10, 30);
        assert_eq!(result.ops(), 0.7481818);
    }

}
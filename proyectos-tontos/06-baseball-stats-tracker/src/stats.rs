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
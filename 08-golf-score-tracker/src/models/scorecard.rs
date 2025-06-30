use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Result;
use crate::utils::validators::{validate_hole_number, validate_par, validate_score};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scorecard {
    pub round_id: Uuid,
    pub player_id: Uuid,
    pub max_holes: u8,
    scores: BTreeMap<u8, u8>,
    pars: BTreeMap<u8, u8>,
}

impl Scorecard {
    pub fn new(player_id: Uuid, max_holes: u8, pars: BTreeMap<u8, u8>) -> Result<Self> {
        for (&hole, &to_par) in &pars {
            validate_hole_number(hole, max_holes)?;
            validate_par(to_par)?;
        }
        Ok(Self {
            round_id: Uuid::new_v4(),
            player_id,
            max_holes,
            scores: BTreeMap::new(),
            pars,
        })
    }

    pub fn record_score(&mut self, hole: u8, strokes:u8) -> Result<()> {
        validate_hole_number(hole, self.max_holes)?;
        let par = *self.pars.get(&hole).expect("par must exist for each hole");
        validate_score(strokes, hole, par)?;
        self.scores.insert(hole, strokes);
        Ok(())
    }

    pub fn get_par (&self, hole: u8) -> Option<u8> {
        self.pars.get(&hole).copied()
    }

    pub fn get_score(&self, hole: u8) -> Option<u8> {
        self.scores.get(&hole).copied()
    }

    pub fn is_complete(&self) -> bool {
        self.scores.len() as u8 == self.max_holes
    }

    pub fn total_strokes(&self) -> Option<u16> {
        self.is_complete().then(|| self.scores.values().copied().map(u16::from).sum())
    }

    pub fn score_relative_to_par(&self) -> Option<i16> {
        if !self.is_complete() {
            return None;
        }
        let total_par: u16 = self.pars.values().copied().map(u16::from).sum();
        let total_strokes: u16 = self.scores.values().copied().map(u16::from).sum();
        Some(total_strokes as i16 - total_par as i16)
    }
}
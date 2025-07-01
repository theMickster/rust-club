//! Player statistics calculation and aggregation.
//!
//! This module provides functionality to calculate comprehensive golf statistics
//! from a collection of scorecards. Statistics include scoring averages, best/worst
//! performances, and hole-by-hole analysis.
//!
//! # Examples
//!
//! ```
//! use golf_score_tracker::{Player, Scorecard, PlayerStatistics};
//! use std::collections::BTreeMap;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let player = Player::new("Tiger Woods", Some(0.0))?;
//! 
//! // Create a simple 9-hole scorecard
//! let mut pars = BTreeMap::new();
//! for hole in 1..=9 {
//!     pars.insert(hole, 4);
//! }
//! 
//! let mut scorecard = Scorecard::new(player.id, 9, pars)?;
//! for hole in 1..=9 {
//!     scorecard.record_score(hole, 4)?;
//! }
//! 
//! // Calculate statistics
//! let stats = PlayerStatistics::from_scorecards(&[scorecard]);
//! assert_eq!(stats.completed_rounds, 1);
//! assert_eq!(stats.average_score, Some(36.0));
//! # Ok(())
//! # }
//! ```
use crate::models::Scorecard;

/// Comprehensive statistics for a player's golf performance.
///
/// This structure aggregates data from multiple scorecards to provide
/// insights into a player's performance trends, consistency, and skill level.
///
/// # Fields
///
/// * `total_rounds` - Total number of scorecards provided (including incomplete)
/// * `completed_rounds` - Number of completed rounds (all holes recorded)
/// * `average_score` - Mean score across completed rounds, `None` if no completed rounds
/// * `best_score` - Lowest total strokes in any completed round
/// * `worst_score` - Highest total strokes in any completed round
/// * `total_under_par` - Cumulative strokes under par (negative value)
/// * `total_over_par` - Cumulative strokes over par (positive value)
/// * `eagles` - Number of holes played 2+ strokes under par
/// * `birdies` - Number of holes played 1 stroke under par
/// * `pars` - Number of holes played at par
/// * `bogeys` - Number of holes played 1 stroke over par
/// * `double_bogeys` - Number of holes played 2+ strokes over par
///
#[derive(Debug, Clone)]
pub struct PlayerStatistics {
pub total_rounds: usize,
    pub completed_rounds: usize,
    pub average_score: Option<f64>,
    pub best_score: Option<u16>,
    pub worst_score: Option<u16>,
    pub total_under_par: i32,
    pub total_over_par: i32,
    pub eagles: usize,
    pub birdies: usize,
    pub pars: usize,
    pub bogeys: usize,
    pub double_bogeys: usize
}

impl PlayerStatistics {
    /// Creates statistics by analyzing a collection of scorecards.
    ///
    /// This method uses iterator patterns and closures to efficiently
    /// calculate comprehensive statistics from raw scorecard data.
    ///
    /// # Arguments
    ///
    /// * `scorecards` - Slice of scorecards to analyze
    ///
    /// # Returns
    ///
    /// A `PlayerStatistics` instance with all fields populated based on
    /// the provided scorecards. Incomplete rounds are counted but excluded
    /// from scoring calculations.
    ///
    /// # Performance
    ///
    /// This method makes multiple passes over the scorecard data using
    /// iterator chains. For large datasets (1000+ rounds), consider
    /// caching the result rather than recalculating frequently.    
    pub fn from_scorecards( scorecards: &[Scorecard]) -> Self {
        let total_rounds = scorecards.len();

        let completed_scorecards: Vec<&Scorecard> = scorecards.iter().filter(|x| x.is_complete()).collect();
        let completed_rounds = completed_scorecards.len();
        let average_score = if completed_rounds > 0 {
            let total: u16 = completed_scorecards.iter().filter_map(|x| x.total_strokes()).sum();
            Some(total as f64 / completed_rounds as f64)
        }
        else {
            None
        };
        let best_score = completed_scorecards.iter().filter_map(|x| x.total_strokes()).min();
        let worst_score = completed_scorecards.iter().filter_map(|x| x.total_strokes()).max();
        let relative_scores: Vec<i16> = completed_scorecards.iter().filter_map(|x| x.score_relative_to_par()).collect();
        let total_under_par = relative_scores.iter().filter(|&&score| score < 0).map(|&score| score as i32).sum();
        let total_over_par = relative_scores.iter().filter(|&&score| score > 0).map(|&score| score as i32).sum();

        let (eagles, birdies, pars, bogeys, double_bogeys) = Self::calculate_hole_statistics(&completed_scorecards);

        Self {
            total_rounds,
            completed_rounds,
            average_score,
            best_score,
            worst_score,
            total_under_par,
            total_over_par,
            eagles,
            birdies,
            pars,
            bogeys,
            double_bogeys
        }
    }

    
    fn calculate_hole_statistics(scorecards: &[&Scorecard]) -> (usize, usize, usize, usize, usize) {
        let mut eagles = 0;
        let mut birdies = 0;
        let mut pars = 0;
        let mut bogeys = 0;
        let mut double_bogeys = 0;

        for scorecard in scorecards {
            for hole in 1..=scorecard.max_holes {
                if let Some(strokes) = scorecard.get_score(hole) {
                    if let Some(par) = scorecard.get_par(hole) {
                        let difference = strokes as i8 - par as i8;
                        match difference {
                            ..=-2 => eagles += 1,
                            -1 => birdies += 1,
                            0 => pars += 1,
                            1 => bogeys += 1,
                            2.. => double_bogeys += 1,
                        }
                    }
                }
            }
        }

        (eagles, birdies, pars, bogeys, double_bogeys)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use uuid::Uuid;

    fn create_test_scorecard(player_id: Uuid, scores: Vec<u8>, pars: Vec<u8>) -> Scorecard {
        let mut map_pars = BTreeMap::new();
        for (hole, par) in pars.iter().enumerate() {
            map_pars.insert((hole + 1) as u8, *par);
        } 

        let mut scorecard = Scorecard::new(player_id, scores.len() as u8, map_pars).expect("Failed to create a test scorecard");
        
        for (hole, strokes) in scores.iter().enumerate() {
            scorecard.record_score((hole + 1) as u8, *strokes)
                .expect(&format!("Failed to record score for hole {}", hole + 1));
        }
        
        scorecard
    }

    #[test]
    fn empty_scorecard() {
        let scorecards: Vec<Scorecard> = vec![];
        let stats = PlayerStatistics::from_scorecards(&scorecards);
        
        assert_eq!(stats.total_rounds, 0);
        assert_eq!(stats.completed_rounds, 0);
        assert_eq!(stats.average_score, None);
        assert_eq!(stats.best_score, None);
        assert_eq!(stats.worst_score, None);
    }

    #[test]
    fn single_scorecard() {
        let player_id = Uuid::new_v4();
        
        // 9-hole round: par 4s, player shoots 36 (even par)
        let scores = vec![4, 4, 4, 4, 4, 4, 4, 4, 4];
        let pars = vec![4, 4, 4, 4, 4, 4, 4, 4, 4];
        
        let scorecard = create_test_scorecard(player_id, scores, pars);
        let stats = PlayerStatistics::from_scorecards(&[scorecard]);
        
        assert_eq!(stats.total_rounds, 1);
        assert_eq!(stats.completed_rounds, 1);
        assert_eq!(stats.average_score, Some(36.0));
        assert_eq!(stats.best_score, Some(36));
        assert_eq!(stats.worst_score, Some(36));
        assert_eq!(stats.total_under_par, 0);
        assert_eq!(stats.total_over_par, 0);
        assert_eq!(stats.pars, 9);
    }

    #[test]
    fn calculates_average() {
        let player_id = Uuid::new_v4();
        let card1 = create_test_scorecard(
            player_id,
            vec![4, 3, 4, 3, 4, 4, 3, 4, 4],
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4],
        );
        let card2 = create_test_scorecard(
            player_id,
            vec![5, 4, 5, 4, 4, 4, 4, 5, 3],
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4],
        );
        
        let stats = PlayerStatistics::from_scorecards(&[card1, card2]);
        
        assert_eq!(stats.total_rounds, 2);
        assert_eq!(stats.completed_rounds, 2);

        // Average: (33 + 38) / 2 = 35.5
        assert_eq!(stats.average_score, Some(35.5));
        assert_eq!(stats.best_score, Some(33));
        assert_eq!(stats.worst_score, Some(38));
    }

    #[test]
    fn under_par_scorecard() {
        let player_id = Uuid::new_v4();
        let scorecard = create_test_scorecard(
            player_id,
            vec![3, 3, 3, 3, 3, 4, 4, 4, 4], 
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4],
        );
        
        let stats = PlayerStatistics::from_scorecards(&[scorecard]);
        
        assert_eq!(stats.total_under_par, -5);
        assert_eq!(stats.total_over_par, 0);
    }

    #[test]
    fn over_par_scorecard() {
        let player_id = Uuid::new_v4();
        let scorecard = create_test_scorecard(
            player_id,
            vec![5, 5, 5, 5, 5, 4, 4, 4, 4],
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4],
        );
        
        let stats = PlayerStatistics::from_scorecards(&[scorecard]);
        
        assert_eq!(stats.total_under_par, 0);
        assert_eq!(stats.total_over_par, 5);
    }    

    #[test]
    fn individual_hole_statistics() {
        let player_id = Uuid::new_v4();
        
        let scorecard = create_test_scorecard(
            player_id,
            vec![
                2,  // Eagle (par 4)
                3,  // Birdie (par 4)
                4,  // Par (par 4)
                5,  // Bogey (par 4)
                6,  // Double bogey (par 4)
                3,  // Par (par 3)
                4,  // Birdie (par 5)
                5,  // Par (par 5)
                4,  // Par (par 4)
            ],
            vec![4, 4, 4, 4, 4, 3, 5, 5, 4],
        );
        
        let stats = PlayerStatistics::from_scorecards(&[scorecard]);
        
        assert_eq!(stats.eagles, 1);
        assert_eq!(stats.birdies, 2);
        assert_eq!(stats.pars, 4);
        assert_eq!(stats.bogeys, 1);
        assert_eq!(stats.double_bogeys, 1);
    }    

    #[test]
    fn test_mixed_performance_rounds() {
        let player_id = Uuid::new_v4();
        let great_round = create_test_scorecard(
            player_id,
            vec![3, 3, 3, 4, 4, 4, 4, 4, 4], 
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4], 
        );
        let average_round = create_test_scorecard(
            player_id,
            vec![4, 4, 5, 4, 4, 4, 4, 4, 4], 
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4], 
        );
        
        let poor_round = create_test_scorecard(
            player_id,
            vec![5, 5, 5, 4, 4, 5, 4, 5, 4], 
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4],  
        );
        
        let stats = PlayerStatistics::from_scorecards(&[great_round, average_round, poor_round]);
        
        assert_eq!(stats.total_rounds, 3);
        assert_eq!(stats.completed_rounds, 3);
        assert_eq!(stats.best_score, Some(33));
        assert_eq!(stats.worst_score, Some(41));
        // Average: (33 + 37 + 41) / 3 = 37.0
        assert_eq!(stats.average_score, Some(37.0));
        assert_eq!(stats.total_under_par, -3);
        assert_eq!(stats.total_over_par, 6);
    }    

    #[test]
    fn statistics_with_different_course_lengths() {
        let player_id = Uuid::new_v4();        
        let nine_hole = create_test_scorecard(
            player_id,
            vec![3, 4, 4, 4, 4, 4, 4, 4, 5],
            vec![3, 4, 4, 4, 4, 4, 4, 4, 5],
        );
        let eighteen_hole = create_test_scorecard(
            player_id,
            vec![4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 4],
            vec![4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 4],
        );
        
        let results = PlayerStatistics::from_scorecards(&[nine_hole, eighteen_hole]);
        
        assert_eq!(results.total_rounds, 2);
        assert_eq!(results.completed_rounds, 2);
        assert_eq!(results.pars, 27); 
    }
}
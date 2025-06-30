//! Advanced Statistics Example
//! 
//! This example demonstrates:
//! - Creating multiple rounds for a player
//! - Using closures and iterators
//! - Calculating comprehensive statistics
//! - Pattern matching and functional programming
//!
//! Run with: cargo run --example advanced_stats

use std::collections::BTreeMap;
use golf_score_tracker::{Player, Scorecard, PlayerStatistics};


fn main() -> anyhow::Result<()> {
    println!("📊 Golf Score Tracker - Advanced Statistics Example\n");

    let player_jordan = Player::new("Jordan Spieth", Some(1.2))?;
    println!("Player: {} (Handicap: {:?})\n", player_jordan.name, player_jordan.handicap);

    let pars = create_tournament_course();
    let mut scorecards = Vec::new();

     // Round 1: Excellent round
    scorecards.push(create_round(&player_jordan, &pars, generate_scores(-4))?);
    
    // Round 2: Good round
    scorecards.push(create_round(&player_jordan, &pars, generate_scores(-2))?);
    
    // Round 3: Struggled
    scorecards.push(create_round(&player_jordan, &pars, generate_scores(4))?);
    
    // Round 4: Back to form for comeback victory
    scorecards.push(create_round(&player_jordan, &pars, generate_scores(-3))?);

    println!("\n═══════════════════════════════════════");
    println!("📈 Tournament Statistics");
    println!("═══════════════════════════════════════\n");
    
    let stats = PlayerStatistics::from_scorecards(&scorecards);
    
    println!("Total Rounds: {}", stats.total_rounds);
    println!("Completed Rounds: {}", stats.completed_rounds);
    
    if let Some(avg) = stats.average_score {
        println!("Average Score: {:.2}", avg);
    }
    
    if let Some(best) = stats.best_score {
        println!("Best Round: {} strokes", best);
    }
    
    if let Some(worst) = stats.worst_score {
        println!("Worst Round: {} strokes", worst);
    }

    println!("\nRelative to Par:");
    println!("  Total Under Par: {}", stats.total_under_par);
    println!("  Total Over Par: {}", stats.total_over_par);
    
    println!("\nHole Performance:");
    println!("  🦅 Eagles: {}", stats.eagles);
    println!("  🐦 Birdies: {}", stats.birdies);
    println!("  ⛳ Pars: {}", stats.pars);
    println!("  🤷🏼 Bogeys: {}", stats.bogeys);
    println!("  🔥 Double Bogeys+: {}", stats.double_bogeys);
    
    println!("\n═══════════════════════════════════════");
    println!("🔍 Advanced Analysis (Using Closures)");
    println!("═══════════════════════════════════════\n");

    let under_par_rounds: Vec<_> = scorecards
        .iter()
        .filter(|card| card.score_relative_to_par().unwrap_or(0) < 0)
        .collect();
    println!("Rounds under par: {}", under_par_rounds.len());

    let scores: Vec<i16> = scorecards
        .iter()
        .filter_map(|card| card.score_relative_to_par())
        .collect();
    
    let mean: f64 = scores.iter().sum::<i16>() as f64 / scores.len() as f64;
    let variance: f64 = scores
        .iter()
        .map(|&score| {
            let diff = score as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / scores.len() as f64;
    let std_dev = variance.sqrt();
    
    println!("🤔🧐 Scoring Consistency (Std Dev): {:.2}", std_dev);
    
    // Find best and worst holes
    let mut hole_performance: BTreeMap<u8, Vec<i8>> = BTreeMap::new();
    for card in &scorecards {
        for hole in 1..=18 {
            if let (Some(strokes), Some(par)) = (card.get_score(hole), card.get_par(hole)) {
                let diff = strokes as i8 - par as i8;
                hole_performance.entry(hole).or_insert_with(Vec::new).push(diff);
            }
        }
    }
    
    let avg_by_hole: BTreeMap<u8, f64> = hole_performance
        .iter()
        .map(|(&hole, diffs)| {
            let avg = diffs.iter().sum::<i8>() as f64 / diffs.len() as f64;
            (hole, avg)
        })
        .collect();
    
    let best_hole = avg_by_hole
        .iter()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    
    let worst_hole = avg_by_hole
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    println!("\n═══════════════════════════════════════");
    println!("🏆  Best Hole: #{} (Avg: {:.2} relative to par)", best_hole.0, best_hole.1);
    println!("🗑️  Worst Hole: #{} (Avg: {:.2} relative to par)", worst_hole.0, worst_hole.1);
    println!("\n═══════════════════════════════════════");
    println!("🎆  Example complete! Explore the code to see:");
    println!("    - Closure usage with .filter(), .map(), .filter_map()");
    println!("    - Iterator chaining for complex calculations");
    println!("    - Pattern matching with match expressions");
    println!("    - Working with Options and Results\n");
    
    Ok(())

}

/// Create a standard 18-hole course layout
fn create_tournament_course() -> BTreeMap<u8, u8> {
    let layout = vec![4, 5, 4, 3, 4, 4, 3, 5, 4, 4, 4, 5, 3, 4, 4, 3, 5, 4];
    layout.iter().enumerate()
        .map(|(i, &par)| ((i + 1) as u8, par))
        .collect()
}


/// Generate realistic scores that average to target relative to par
/// Create mock data using all par 4s and adjusting using the modulo pattern.
fn generate_scores(target_relative: i16) -> Vec<u8> {
    let base_par = 72;
    let target_total = (base_par as i16 + target_relative) as u8;
    let mut scores = vec![4; 18];     
    let adjustment = target_total as i16 - 72;
    let mut remaining = adjustment;
        
    for i in 0..18 {
        if remaining < 0 && i % 3 == 0 {
            scores[i] = 3;
            remaining += 1;
        } else if remaining > 0 && i % 4 == 0 {
            scores[i] = 5;
            remaining -= 1;
        }
    }
    
    scores
}

/// Create a round for a player with given scores
fn create_round(player: &Player, pars: &BTreeMap<u8, u8>, scores: Vec<u8>) -> anyhow::Result<Scorecard> {
    let mut card = Scorecard::new(player.id, 18, pars.clone())?;
    for (hole, &strokes) in scores.iter().enumerate() {
        card.record_score((hole + 1) as u8, strokes)?;
    }
    Ok(card)
}
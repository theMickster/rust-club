//! Basic Golf Tracker Usage Example
//! 
//! This example demonstrates:
//! - Creating players
//! - Creating scorecards
//! - Recording scores
//! - Viewing results
//!
//! Run with: cargo run --example basic_usage

use std::collections::BTreeMap;
use golf_score_tracker::{Player, Scorecard, FileRepository, Repository};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("🏌️ Golf Score Tracker - Basic Usage Example\n");
    
    let temp_dir = PathBuf::from("./examples_data");
    std::fs::create_dir_all(&temp_dir)?;
    let mut repo = FileRepository::new(temp_dir)?;
        
    println!("Step 1: Creating players...");
    let player_rory = Player::new("Rory McIlroy", Some(0.0))?;
    let player_scottie = Player::new("Scottie Scheffler", Some(2.5))?;
    
    repo.save_player(&player_rory)?;
    repo.save_player(&player_scottie)?;
    println!("✅ Created {} (ID: {})", player_rory.name, player_rory.id);
    println!("✅ Created {} (ID: {})\n", player_scottie.name, player_scottie.id);
    
    // Step 2: Create a 9-hole course
    println!("Step 2: Creating a 9-hole course...");
    let mut pars = BTreeMap::new();
    for hole in 1..=9 {
        let par = match hole {
            2 | 5 | 8 => 5,  // Par 5s
            3 | 7 => 3,      // Par 3s
            _ => 4,          // Par 4s
        };
        pars.insert(hole, par);
    }
    println!("✅ Course layout: {:?}\n", pars);
    
    // Step 3: Create scorecards
    println!("Step 3: Creating scorecards...");
    let mut rory_card = Scorecard::new(player_rory.id, 9, pars.clone())?;
    let mut scottie_card = Scorecard::new(player_scottie.id, 9, pars)?;
    println!("✅ Created scorecards\n");
    
    // Step 4: Record Rory's round (excellent!)
    println!("Step 4: Recording Rory's scores...");
    let rory_scores = vec![3, 4, 2, 3, 4, 4, 3, 4, 3];  // 30 total, -6 under par
    for (hole, strokes) in rory_scores.iter().enumerate() {
        rory_card.record_score((hole + 1) as u8, *strokes)?;
    }
    println!("✅ Rory's round: {:?}", rory_scores);
    
    // Step 5: Record Scottie's round (good, but not as good)
    println!("\nStep 5: Recording Scottie's scores...");
    let scottie_scores = vec![4, 5, 3, 4, 5, 4, 3, 5, 4];  // 37 total, +1 over par
    for (hole, strokes) in scottie_scores.iter().enumerate() {
        scottie_card.record_score((hole + 1) as u8, *strokes)?;
    }
    println!("✅ Scottie's round: {:?}", scottie_scores);
    
    // Step 6: Display results
    println!("\n📊 Final Results:");
    println!("═══════════════════════════════════════");
    
    display_scorecard(&player_rory.name, &rory_card);
    display_scorecard(&player_scottie.name, &scottie_card);
    
    // Step 7: Save to repository
    println!("\n💾 Saving to repository...");
    repo.save_scorecard(&rory_card)?;
    repo.save_scorecard(&scottie_card)?;
    println!("✅ All data saved!\n");
    
    Ok(())
}

fn display_scorecard(player_name: &str, scorecard: &Scorecard) {
    println!("\n{}", player_name);
    println!("─────────────────────────────────────");
    
    if let Some(total) = scorecard.total_strokes() {
        println!("Total strokes: {}", total);
    }
    
    if let Some(relative) = scorecard.score_relative_to_par() {
        let status = match relative {
            r if r < 0 => format!("{} under par 🔥", -r),
            r if r > 0 => format!("{} over par 💩", r),
            _ => "even par".to_string(),
        };
        println!("Score: {}", status);
    }
    
    println!("Complete: {}", if scorecard.is_complete() { "✅" } else { "⏳" });
}
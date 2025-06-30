use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Parser;

use golf_score_tracker::{FileRepository, Player, PlayerStatistics, Repository, Scorecard};
use golf_score_tracker::ui::{Cli, Commands};
use golf_score_tracker::utils::{get_course_pars, list_available_courses};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_dir = PathBuf::from("./golf_data");
    let mut repo = FileRepository::new(data_dir)
        .context("Failed to initialize repository")?;

    match cli.command {
        Commands::AddPlayer { name, handicap } => {
            let player = Player::new(name, handicap)
                .context("Failed to create player")?;
            repo.save_player(&player)
                .context("Failed to save player")?;
            println!("Player created: {} (ID: {})", player.name, player.id);
        }

        Commands::ListPlayers => {
            let players = repo.list_players().context("Failed to list players")?;
            if players.is_empty() {
                println!("No players found");
            } else {
                for player in players {
                    println!("{} - {} (handicap: {:?})", 
                        player.id, player.name, player.handicap);
                }
            }
        }

        Commands::RecordScore { player_id, hole, strokes } => {
            println!("Recording score: player={}, hole={}, strokes={}", 
                player_id, hole, strokes);
            println!("(Full scorecard implementation coming next)");
        }

        Commands::CreateScorecard { player_id, holes, course } => {
            let player = repo.get_player(&player_id)
                .context("Failed to get player")?
                .ok_or_else(|| anyhow::anyhow!("Player {} not found", player_id))?;
            
            let pars = match &course {
                Some(course_name) => get_course_pars(&course_name, holes),
                None => get_course_pars("standard", holes),
            };
            
            let scorecard = Scorecard::new(player_id, holes, pars)
                .context("Failed to create scorecard")?;
            
            repo.save_scorecard(&scorecard)
                .context("Failed to save scorecard")?;
            
            let course_display = course.unwrap_or_else(|| "Standard".to_string());
            println!("✅ Scorecard created for {} on {} course (Round ID: {})", 
                player.name, course_display, scorecard.round_id);
        }

        Commands::ListScorecards { player_id } => {
            let scorecards = if let Some(pid) = player_id {
                repo.get_scorecards_by_player(&pid)
                    .context("Failed to get scorecards for player")?
            } else {
                repo.list_scorecards()
                    .context("Failed to list scorecards")?
            };
            
            if scorecards.is_empty() {
                println!("No scorecards found");
            } else {
                println!("📋 Scorecards:");
                for sc in scorecards {
                    let status = if sc.is_complete() { "✅" } else { "⏳" };
                    println!("  {} Round {} - Player {}", 
                        status, sc.round_id, sc.player_id);
                }
            }
        }

        Commands::ShowScorecard { round_id } => {
            let scorecard = repo.get_scorecard(&round_id)
                .context("Failed to get scorecard")?
                .ok_or_else(|| anyhow::anyhow!("Scorecard {} not found", round_id))?;
            
            println!("📊 Scorecard for round {}", round_id);
            println!("   Player: {}", scorecard.player_id);
            println!("   Complete: {}", scorecard.is_complete());
            
            if let Some(total) = scorecard.total_strokes() {
                println!("   Total strokes: {}", total);
            }
            if let Some(relative) = scorecard.score_relative_to_par() {
                println!("   Relative to par: {:+}", relative);
            }
        }        

        Commands::ListCourses => {
            let courses = list_available_courses();
            println!("🏌️ Available courses:");
            for course in courses {
                println!("  • {}", course);
            }
            println!("\nUse --course <name> when creating a scorecard");
        }
    
        Commands::ShowPlayerStatistics {player_id} => {
            let player = repo.get_player(&player_id)
                .context("Failed to retrieve player")?
                .ok_or_else(|| anyhow::anyhow!("Player {} not found", player_id))?;

            let scorecards = repo.get_scorecards_by_player(&player_id)
                .context("Failed to retrieve scorecards")?;

            if scorecards.is_empty() {
                println!("No scorecards found for player {}", player.name);
                return Ok(());
            }
            let stats = PlayerStatistics::from_scorecards(&scorecards);
            
            println!("📊 Statistics for {}", player.name);
            println!("   ⛳️ Total rounds: {}", stats.total_rounds);
            println!("   🏁 Completed rounds: {}", stats.completed_rounds);

            if let Some(avg) = stats.average_score {
                println!("   ⚖️ Average score: {:.2}", avg);
            }
            
            if let Some(best) = stats.best_score {
                println!("   🏆 Best score: {}", best);
            }
            
            if let Some(worst) = stats.worst_score {
                println!("   🆘 Worst score: {}", worst);
            }

            println!("   Total under par: {}", stats.total_under_par);
            println!("   Total over par: {}", stats.total_over_par);
            println!("\n   Hole Performance:");
            println!("      Eagles: {}", stats.eagles);
            println!("      Birdies: {}", stats.birdies);
            println!("      Pars: {}", stats.pars);
            println!("      Bogeys: {}", stats.bogeys);
            println!("      Double bogeys+: {}", stats.double_bogeys);
        }
    }

    Ok(())
}

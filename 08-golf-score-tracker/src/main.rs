use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Parser;

use golf_score_tracker::{FileRepository, Player, Repository};
use golf_score_tracker::ui::{Cli, Commands};

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
            let players = repo.list_players()
                .context("Failed to list players")?;
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
        Commands::ShowScorecard { round_id } => {
            let scorecard = repo.get_scorecard(&round_id)
                .context("Failed to retrieve scorecard")?;
            match scorecard {
                Some(sc) => println!("Scorecard: {:?}", sc),
                None => println!("Scorecard not found"),
            }
        }
        Commands::ListScorecards => {
            let scorecards = repo.list_scorecards()
                .context("Failed to list scorecards")?;
            if scorecards.is_empty() {
                println!("No scorecards found");
            } else {
                for sc in scorecards {
                    println!("Round {} - Player {}", sc.round_id, sc.player_id);
                }
            }
        }
    }

    Ok(())
}

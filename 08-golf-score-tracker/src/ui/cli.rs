use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "golf-tracker")]
#[command(about = "Track golf scores for players", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    AddPlayer {
        name: String,
        #[arg(short = 'c', long)]
        handicap: Option<f64>,
    },

    ListPlayers,
    
    CreateScorecard {
        player_id: Uuid,
        #[arg(short = 'n', long, default_value = "18")]
        holes: u8,
        #[arg(short = 'c', long, value_name = "COURSE")]
        course: Option<String>,
    },
    
    RecordScore {
        player_id: Uuid,
        hole: u8,
        strokes: u8,
    },
    
    ShowScorecard {
        round_id: Uuid,
    },
    
    ListScorecards {
        #[arg(short, long)]
        player_id: Option<Uuid>,
    },
    
    ListCourses,
}
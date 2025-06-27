pub mod error;
pub mod models;
pub mod storage;
pub mod ui;
pub mod utils;

pub use error::{GolfError, Result};
pub use models::{Hole, Player, Round, Scorecard};
pub use storage::{FileRepository, Repository};
//! Repository pattern implementation for golf score tracking.
//!
//! This module provides a trait-based abstraction for data persistence,
//! along with a file system-based implementation.

use std::path::PathBuf;
use uuid::Uuid;

use crate::error::Result;
use crate::models::{Player, Scorecard};

/// Defines the contract for persisting and retrieving golf score data.
///
/// This trait abstracts the storage mechanism, allowing for different
/// implementations (file system, database, in-memory, etc.).
pub trait Repository {
    /// Saves a player to the repository.
    ///
    /// # Arguments
    ///
    /// * `player` - The player to save
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the player was saved successfully
    /// * `Err` if an I/O or serialization error occurred
    fn save_player(&mut self, player: &Player) -> Result<()>;
    
    /// Retrieves a player by their unique identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the player to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(Some(player))` if the player exists
    /// * `Ok(None)` if no player with the given ID exists
    /// * `Err` if an I/O or deserialization error occurred
    fn get_player(&self, id: &Uuid) -> Result<Option<Player>>;
    
    /// Lists all players in the repository.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Player>)` containing all players
    /// * `Err` if an I/O or deserialization error occurred
    fn list_players(&self) -> Result<Vec<Player>>;
    
    /// Saves a scorecard to the repository.
    ///
    /// # Arguments
    ///
    /// * `scorecard` - The scorecard to save
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the scorecard was saved successfully
    /// * `Err` if an I/O or serialization error occurred
    fn save_scorecard(&mut self, scorecard: &Scorecard) -> Result<()>;
    
    /// Retrieves a scorecard by its round identifier.
    ///
    /// # Arguments
    ///
    /// * `round_id` - The UUID of the round to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(Some(scorecard))` if the scorecard exists
    /// * `Ok(None)` if no scorecard with the given round ID exists
    /// * `Err` if an I/O or deserialization error occurred
    fn get_scorecard(&self, round_id: &Uuid) -> Result<Option<Scorecard>>;
    fn get_scorecards_by_player( &self, player_id: &Uuid) -> Result<Vec<Scorecard>>;
    fn list_scorecards(&self) -> Result<Vec<Scorecard>>;
}

/// File system-based implementation of the Repository trait.
///
/// Stores players and scorecards as JSON files in separate subdirectories.
/// Each entity is stored in a file named by its UUID with a `.json` extension.
///
/// # Directory Structure
///
/// ```text
/// base_path/
/// ├── players/
/// │   ├── {uuid}.json
/// │   └── ...
/// └── scorecards/
///     ├── {uuid}.json
///     └── ...
/// ```
pub struct FileRepository {
    base_path: PathBuf,
}

impl FileRepository {
    /// Creates a new FileRepository with the specified base path.
    ///
    /// This method will create the base directory if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The root directory for storing data files
    ///
    /// # Returns
    ///
    /// * `Ok(FileRepository)` if the repository was created successfully
    /// * `Err` if the base directory could not be created
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use golf_score_tracker::storage::FileRepository;
    ///
    /// let repo = FileRepository::new(PathBuf::from("./data"))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(base_path: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
    }

    /// Returns the file system path for a player file.
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the player
    ///
    /// # Returns
    ///
    /// The path to the player's JSON file
    fn player_path(&self, id: &Uuid) -> PathBuf {
        self.base_path.join("players").join(format!("{}.json", id))
    }

    /// Returns the file system path for a scorecard file.
    ///
    /// # Arguments
    ///
    /// * `round_id` - The UUID of the round
    ///
    /// # Returns
    ///
    /// The path to the scorecard's JSON file
    fn scorecard_path(&self, round_id: &Uuid) -> PathBuf {
        self.base_path.join("scorecards").join(format!("{}.json", round_id))
    }
}

impl Repository for FileRepository {
    fn save_player(&mut self, player: &Player) -> Result<()> {
        let path = self.player_path(&player.id);
        std::fs::create_dir_all(path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(player)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    fn get_player(&self, id: &Uuid) -> Result<Option<Player>> {
        let path = self.player_path(id);
        if !path.exists() {
            return Ok(None);
        }
        let json = std::fs::read_to_string(path)?;
        let player = serde_json::from_str(&json)?;
        Ok(Some(player))
    }

    fn list_players(&self) -> Result<Vec<Player>> {
        let dir = self.base_path.join("players");
        if !dir.exists() {
            return Ok(vec![]);
        }

        let mut players = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let json = std::fs::read_to_string(entry.path())?;
            let player: Player = serde_json::from_str(&json)?;
            players.push(player);
        }
        Ok(players)
    }

    fn save_scorecard(&mut self, scorecard: &Scorecard) -> Result<()> {
        let path = self.scorecard_path(&scorecard.round_id);
        std::fs::create_dir_all(path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(scorecard)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    fn get_scorecard(&self, round_id: &Uuid) -> Result<Option<Scorecard>> {
        let path = self.scorecard_path(round_id);
        if !path.exists() {
            return Ok(None);
        }
        let json = std::fs::read_to_string(path)?;
        let scorecard = serde_json::from_str(&json)?;
        Ok(Some(scorecard))
    }

    fn list_scorecards(&self) -> Result<Vec<Scorecard>> {
        let dir = self.base_path.join("scorecards");
        if !dir.exists() {
            return Ok(vec![]);
        }

        let mut scorecards = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let json = std::fs::read_to_string(entry.path())?;
            let scorecard: Scorecard = serde_json::from_str(&json)?;
            scorecards.push(scorecard);
        }
        Ok(scorecards)
    }

    fn get_scorecards_by_player(&self, player_id: &Uuid) -> Result<Vec<Scorecard>> {
        let results = self.list_scorecards()?;
        Ok(results.into_iter().filter(|x | &x.player_id == player_id).collect())
    }

}
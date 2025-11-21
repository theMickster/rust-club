use std::path::PathBuf;
use uuid::Uuid;

use crate::error::Result;
use crate::models::{Player, Scorecard};

pub trait Repository {
    fn save_player(&mut self, player: &Player) -> Result<()>;
    fn get_player(&self, id: &Uuid) -> Result<Option<Player>>;
    fn list_players(&self) -> Result<Vec<Player>>;
    
    fn save_scorecard(&mut self, scorecard: &Scorecard) -> Result<()>;
    fn get_scorecard(&self, round_id: &Uuid) -> Result<Option<Scorecard>>;
    fn get_scorecards_by_player( &self, player_id: &Uuid) -> Result<Vec<Scorecard>>;
    fn list_scorecards(&self) -> Result<Vec<Scorecard>>;
}

pub struct FileRepository {
    base_path: PathBuf,
}

impl FileRepository {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
    }

    fn player_path(&self, id: &Uuid) -> PathBuf {
        self.base_path.join("players").join(format!("{}.json", id))
    }

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
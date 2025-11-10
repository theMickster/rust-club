use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{GolfError, Result};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub handicap: Option<f64>
}

impl Player{
    pub fn new(name: impl AsRef<str>, handicap: Option<f64>) -> Result<Self> {
        let name = name.as_ref().trim().to_string();
        if name.is_empty() {
            return Err(GolfError::custom("Player name cannot be empty"));
        }
        Ok(Self {id: Uuid::new_v4(), name, handicap})
    }
}
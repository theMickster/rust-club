use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Round {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub course: String,    
    pub max_holes: u8
}

impl Round {
    pub fn new(course: impl AsRef<str>, max_holes: u8) -> Self {
        Self {
            id: Uuid::new_v4(),
            date: Utc::now(),
            course: course.as_ref().trim().to_string(),
            max_holes
        }
    }
}
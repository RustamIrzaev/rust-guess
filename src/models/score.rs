use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub name: String,
    pub tries: i32,
    pub started_at: DateTime<Local>,
    pub completed_at: DateTime<Local>,
    pub completed_for_ms: i64,
    pub number_range: String,
    pub is_hard_mode: bool,
}
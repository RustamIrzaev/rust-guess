use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub min_number: i32,
    pub max_number: i32,
    pub generated_number: i32,
    pub current_guess_response: String,
    pub is_game_over: bool,
    pub game_started_at: DateTime<Local>,
    pub game_completed_at: DateTime<Local>,
    pub is_hard_mode: bool,
}
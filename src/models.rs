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

pub enum CurrentScreen {
    Game,
    Menu,
    Leaderboard,
}

#[derive(Serialize, Deserialize)]
pub struct GameMove {
    pub move_done_at: DateTime<Local>,
    pub user_value: i32,
}

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

pub enum UserInputMode {
    InputNumber,
    InputName,
}

pub struct UserInputInfo {
    pub character_index: usize,
    pub input: String,
}
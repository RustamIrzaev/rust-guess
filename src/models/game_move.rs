use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameMove {
    pub move_done_at: DateTime<Local>,
    pub user_value: i32,
}
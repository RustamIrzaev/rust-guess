use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Write};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

const LEADERBOARD_FILE_NAME: &'static str = "data.rom";

#[derive(Serialize, Deserialize)]
pub(crate) struct Score {
    pub(crate) name: String,
    pub(crate) tries: i32,
    pub(crate) completed_at: DateTime<Local>,
    pub(crate) completed_for_msec: i64,
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name {}\ttries:{}\tcompleted_in:{}ms", self.name, self.tries, self.completed_for_msec)
    }
}

pub fn load_scores() -> Vec<Score> {
    let file = match File::open(LEADERBOARD_FILE_NAME) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

pub fn save_scores(scores: &Vec<Score>) {
    let json = match serde_json::to_string(scores) {
        Ok(r) => r,
        Err(_) => return
    };

    match File::create(LEADERBOARD_FILE_NAME)
        .and_then(|mut file| Write::write_all(&mut file, json.as_bytes())) {
        Ok(_) => {},
        Err(_) => println!("Save to file failed"),
    }
}
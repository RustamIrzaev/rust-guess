use std::fs::File;
use std::io::{BufReader, Write};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

const LEADERBOARD_FILE_NAME: &'static str = "data.rom";

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub name: String,
    pub tries: i32,
    pub started_at: DateTime<Local>,
    pub completed_at: DateTime<Local>,
    pub completed_for_ms: i64,
    pub number_range: String,
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

pub fn add_score(name: String, tries: i32, number_range: String,
                 started_at: DateTime<Local>, completed_at: DateTime<Local>,
                 completed_for_ms: i64) {
    let mut scores = load_scores();

    let new_entry = Score {
        name,
        tries,
        started_at,
        completed_at,
        completed_for_ms,
        number_range,
    };

    scores.push(new_entry);
    scores.sort_by_key(|entry| entry.tries);
    save_scores(&scores);
}
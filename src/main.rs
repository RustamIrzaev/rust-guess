use std::cmp::{Ordering, Reverse};
use std::fs::File;
use std::io;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Write};
use rand::Rng;
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
struct Score {
    name: String,
    tries: i32,
    completed_at: DateTime<Local>,
    completed_for_msec: i64,
}

const NUM_MINIMUM: i32 = 1;
const NUM_MAXIMUM: i32 = 100;
const LEADERBOARD_FILE_NAME: &str = "data.rom";

fn main() {
    print!("\x1B[2J\x1B[1;1H"); // clear the console :)

    let mut scores = load_scores();

    let value = rand::thread_rng().gen_range(NUM_MINIMUM..=NUM_MAXIMUM);
    let mut tries = 0;
    let mut number_minimum = NUM_MINIMUM;
    let mut number_maximum = NUM_MAXIMUM;
    let start_time: DateTime<Local> = Local::now();

    println!("Welcome to Guesser the Game");
    println!();
    println!("You have to guess a number from {NUM_MINIMUM} to {NUM_MAXIMUM}");

    loop {
        println!();
        println!("enter a number [{number_minimum}:{number_maximum}]:");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Can't get your number");

        let number: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match number.cmp(&value) {
            Ordering::Greater => {
                tries += 1;
                println!("Number is < than {number}");
                number_maximum = number;
            },
            Ordering::Less => {
                tries += 1;
                println!("Number is > than {number}");
                number_minimum = number;
            },
            Ordering::Equal => {
                let end_time = Local::now();
                let time_diff = end_time.time() - start_time.time();
                let msec_diff = time_diff.num_milliseconds();

                println!();
                println!("YOU WON in {tries} tries");

                let name = ask_for_name();
                let new_entry = Score {
                    name: name.trim().to_owned(),
                    tries,
                    completed_at: end_time,
                    completed_for_msec: msec_diff,
                };

                scores.push(new_entry);
                scores.sort_by_key(|entry| Reverse(entry.tries));
                save_scores(&scores);

                break;
            },
        }
    }
}

fn ask_for_name() -> String {
    println!();
    println!("Enter your name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("waiting for your name");

    return name;
}

fn load_scores() -> Vec<Score> {
    let file = match File::open(LEADERBOARD_FILE_NAME) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);

    let leaderboard: Vec<Score> = match serde_json::from_reader(reader) {
        Ok(r) => r,
        Err(_) => return Vec::new()
    };

    return leaderboard
}

fn save_scores(scores: &Vec<Score>) {
    let json = match serde_json::to_string(scores) {
        Ok(r) => r,
        Err(_) => return
    };

    let mut file = match File::create(LEADERBOARD_FILE_NAME) {
        Ok(f) => f,
        Err(_) => return,
    };

    file.write_all(json.as_bytes()).unwrap();
}
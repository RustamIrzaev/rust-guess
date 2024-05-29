use std::cmp::{Ordering};
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

        tries += 1;

        match number.cmp(&value) {
            Ordering::Greater => {
                println!("Number is < than {number}");
                number_maximum = number;
            },
            Ordering::Less => {
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
                    tries,
                    name: name.trim().to_owned(),
                    completed_at: end_time,
                    completed_for_msec: msec_diff,
                };

                scores.push(new_entry);
                // scores.sort_by_key(|entry| Reverse(entry.tries));
                scores.sort_by_key(|entry| entry.tries);
                save_scores(&scores);

                print_scores(&scores);

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

fn print_scores(scores: &Vec<Score>) {
    // print!("\x1B[2J\x1B[1;1H");
    println!();
    println!("Scores");

    for i in 0..scores.len() {
        let item = scores.get(i).unwrap();
        println!("{}.  {}\t{} tries  {}ms", i+1, item.name, item.tries, item.completed_for_msec);
    }

    let min_tries = scores.iter().map(|x| x.tries).min().unwrap();
    let max_tries = scores.iter().map(|x| x.tries).max().unwrap();
    let sum_tries: i32 = scores.iter().map(|x| x.tries).sum();
    let average_tries = sum_tries as f32 / scores.len() as f32;

    let sum_time: i64 = scores.iter().map(|x| x.completed_for_msec).sum();
    let average_time = sum_time / scores.len() as i64;

    println!();
    println!("Minimum tries: {min_tries}");
    println!("Maximum tries: {max_tries}");
    println!("Average tries: {}", average_tries.round());
    println!();
    println!("Average time per a game (ms): {}", average_time);
}
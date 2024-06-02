mod scores;
mod ui;
mod app;

use std::{io::{Result, stdin}, cmp::Ordering, io};
use rand::Rng;
use chrono::prelude::*;
use crossterm::{event::{self, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, execute};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use ratatui::backend::Backend;
use crate::app::{App, CurrentScreen};

use crate::scores::{load_scores, save_scores, Score};
use crate::ui::ui;

const NUM_MINIMUM: i32 = 1;
const NUM_MAXIMUM: i32 = 100;

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    app.start_game();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    terminal.show_cursor()?;

    if let Ok(_) = res {
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Game => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Quit;
                    }

                    _ => {}
                },
                CurrentScreen::Leaderboard => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Menu;
                    }
                    _ => {},
                }
                CurrentScreen::Menu => match key.code {
                    KeyCode::Up => {
                        if app.get_selected_menu_idx() > 0 {
                            let index = app.get_selected_menu_idx();
                            app.main_menu_item_selected.select(Some(index - 1));
                        }
                    },
                    KeyCode::Down => {
                        if app.get_selected_menu_idx() < app.main_menu_items.len() - 1 {
                            let index = app.get_selected_menu_idx();
                            app.main_menu_item_selected.select(Some(index + 1));
                        }
                    },
                    KeyCode::Enter => {
                        let index = app.get_selected_menu_idx();

                        match index {
                            0 => {
                                app.current_screen = CurrentScreen::Game;
                            },
                            1 => {
                                app.current_screen = CurrentScreen::Leaderboard;
                            },
                            _ => return Ok(false),
                        }
                    }
                    _ => {},
                },
                CurrentScreen::Quit => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Menu;
                        // return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        // return Ok(false);
                        app.current_screen = CurrentScreen::Game;
                        continue;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn main2() {
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

        stdin()
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

    stdin()
        .read_line(&mut name)
        .expect("waiting for your name");

    name
}

fn print_scores(scores: &Vec<Score>) {
    // print!("\x1B[2J\x1B[1;1H");
    println!();
    println!("Scores");

    scores.iter().enumerate().take(10).for_each(|(i, score)| {
        println!("{}.  {}\t{} tries  {}ms", i+1, score.name, score.tries, score.completed_for_msec);
    });

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
    println!("Average time per a game: {}ms", average_time);
}
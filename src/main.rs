mod app;
mod scores;
mod ui;
mod models;

use crate::app::{App};
use crate::ui::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io::{self, Result};
use crate::models::{CurrentScreen, UserInputMode};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(&mut terminal);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<bool> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Game => match key.code {
                    KeyCode::Char('q') => match app.mode {
                        UserInputMode::InputName => continue,
                        _ => {
                            if !app.quit_confirm_popup {
                                app.quit_confirm_popup = !app.quit_confirm_popup;
                            }
                        }
                    },
                    KeyCode::Char('y') => {
                        if app.quit_confirm_popup {
                            app.current_screen = CurrentScreen::Menu;
                            app.quit_confirm_popup = false;
                        }
                    }
                    KeyCode::Char('n') => {
                        if app.quit_confirm_popup {
                            app.quit_confirm_popup = false;
                        }
                    }
                    KeyCode::Char(value) => match app.mode {
                        UserInputMode::InputNumber => {
                            if !app.quit_confirm_popup && value.is_numeric() {
                                app.input_enter_char(value);
                            }
                        }
                        UserInputMode::InputName => {
                            if !app.quit_confirm_popup {
                                app.input_enter_char(value);
                            }
                        }
                    },
                    KeyCode::Backspace => match app.mode {
                        UserInputMode::InputNumber | UserInputMode::InputName => {
                            app.input_delete_char();
                        }
                    },
                    KeyCode::Enter => match app.mode {
                        UserInputMode::InputNumber => {
                            app.input_submit_number();
                        }
                        UserInputMode::InputName => {
                            app.input_submit_name();
                        }
                    },

                    _ => {}
                },
                CurrentScreen::Leaderboard => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Menu;
                    }
                    _ => {}
                },
                CurrentScreen::Menu => match key.code {
                    KeyCode::Up => {
                        if app.get_selected_menu_idx() > 0 {
                            let index = app.get_selected_menu_idx();
                            app.main_menu_item_selected.select(Some(index - 1));
                        }
                    }
                    KeyCode::Down => {
                        if app.get_selected_menu_idx() < app.main_menu_items.len() - 1 {
                            let index = app.get_selected_menu_idx();
                            app.main_menu_item_selected.select(Some(index + 1));
                        }
                    }
                    KeyCode::Enter => {
                        let index = app.get_selected_menu_idx();

                        match index {
                            0 => { //1-100 easy
                                app.current_screen = CurrentScreen::Game;
                                app.start_game(1, 100, false);
                            }
                            1 => { //1-1000 hard
                                app.current_screen = CurrentScreen::Game;
                                app.start_game(1, 100, true);
                            }
                            2 => { //1-1000 easy
                                app.current_screen = CurrentScreen::Game;
                                app.start_game(1, 1000, false);
                            }
                            3 => { //1-1000 hard
                                app.current_screen = CurrentScreen::Game;
                                app.start_game(1, 1000, true);
                            }
                            4 => { //1-1000000 hard
                                app.current_screen = CurrentScreen::Game;
                                app.start_game(1, 1000000, true);
                            }
                            5 => {
                                app.current_screen = CurrentScreen::Leaderboard;
                            }
                            _ => return Ok(false),
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}

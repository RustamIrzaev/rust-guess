mod scores;
mod ui;
mod app;

use std::{
    io::{self, Result},
};
use crossterm::{
    event::{self, KeyCode, KeyEventKind, DisableMouseCapture, EnableMouseCapture, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute};
use ratatui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
};
use crate::app::{App, CurrentScreen, UserInputMode};
use crate::ui::ui;

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
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
                        match app.mode {
                            UserInputMode::InputName => continue,
                            _ => {
                                if !app.quit_confirm_popup {
                                    app.quit_confirm_popup = !app.quit_confirm_popup;
                                }
                            },
                        }
                    },
                    KeyCode::Char('y') => {
                        if app.quit_confirm_popup {
                            app.current_screen = CurrentScreen::Menu;
                            app.quit_confirm_popup = false;
                        }
                    },
                    KeyCode::Char('n') => {
                        if app.quit_confirm_popup {
                            app.quit_confirm_popup = false;
                        }
                    },
                    KeyCode::Char(value) => {
                        match app.mode {
                            UserInputMode::InputNumber => {
                                if !app.quit_confirm_popup && value.is_numeric() {
                                    app.input_enter_char(value);
                                }
                            },
                            UserInputMode::InputName => {
                                if !app.quit_confirm_popup {
                                    app.input_enter_char(value);
                                }
                            },
                        }
                    },
                    KeyCode::Backspace => {
                        match app.mode {
                            UserInputMode::InputNumber => {
                                app.input_delete_char();
                            },
                            _ => {},
                        }
                    },
                    KeyCode::Enter => {
                        match app.mode {
                            UserInputMode::InputNumber => {
                                app.input_submit_number();
                            },
                            UserInputMode::InputName => {
                                app.input_submit_name();
                            },
                        }
                    },

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
                                app.start_game();
                            },
                            1 => {
                                app.current_screen = CurrentScreen::Leaderboard;
                            },
                            _ => return Ok(false),
                        }
                    }
                    _ => {},
                },
            }
        }
    }
}
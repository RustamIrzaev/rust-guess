use chrono::{DateTime, Local};
use rand::Rng;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use crate::{NUM_MAXIMUM, NUM_MINIMUM};

pub enum CurrentScreen {
    Game,
    Menu,
    Leaderboard,
    Quit,
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
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub main_menu_item_selected: ListState,
    pub main_menu_items: Vec<String>,
    pub game_info: GameInfo,
    pub user_input_history: Vec::<GameMove>,
    pub quit_confirm_popup: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
            quit_confirm_popup: false,
            user_input_history: Vec::<GameMove>::new(),
            game_info: GameInfo {
                min_number: NUM_MINIMUM,
                max_number: NUM_MAXIMUM,
                generated_number: 0,
            },
            main_menu_item_selected: ListState::default().with_selected(Some(0)),
            main_menu_items: vec![
                "Start game".to_string(),
                "Leaderboard".to_string(),
                "Quit".to_string(),
            ]
        }
    }
    
    pub fn get_selected_menu_idx(&self) -> usize {
        self.main_menu_item_selected.selected().unwrap_or(0)
    }

    pub fn start_game(&mut self) {
        self.game_info.generated_number = rand::thread_rng()
            .gen_range(NUM_MINIMUM..=NUM_MAXIMUM);

        self.user_input_history.push(GameMove {
            user_value: 10,
            move_done_at: Local::now(),
        });

        self.user_input_history.push(GameMove {
            user_value: 22,
            move_done_at: Local::now(),
        });
    }
}
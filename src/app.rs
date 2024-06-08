use chrono::{DateTime, Local};
use rand::Rng;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use crate::{NUM_MAXIMUM, NUM_MINIMUM};

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
}

pub enum UserInputMode {
    ReadOnly,
    InputNumber,
    InputName,
}

pub struct UserInputInfo {
    pub character_index: usize,
    pub input: String,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub main_menu_item_selected: ListState,
    pub main_menu_items: Vec<String>,
    pub game_info: GameInfo,
    pub user_input_history: Vec::<GameMove>,
    pub quit_confirm_popup: bool,
    pub user_input_info: UserInputInfo,
    pub mode: UserInputMode,
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
            ],
            user_input_info: UserInputInfo {
                character_index: 0,
                input: String::new(),
            },
            mode: UserInputMode::InputNumber,
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

    fn input_move_cursor_left(&mut self) {
        let cursor = self.user_input_info.character_index.saturating_sub(1);
        self.user_input_info.character_index = self.input_clamp_cursor(cursor);
    }

    fn input_move_cursor_right(&mut self) {
        let cursor = self.user_input_info.character_index.saturating_add(1);
        self.user_input_info.character_index = self.input_clamp_cursor(cursor);
    }

    fn input_clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.user_input_info.input.chars().count())
    }

    fn input_reset_cursor(&mut self) {
        self.user_input_info.character_index = 0;
    }

    pub fn input_enter_char(&mut self, new_char: char) {
        let index = self.input_byte_index();

        self.user_input_info.input.insert(index, new_char);
        self.input_move_cursor_right();
    }

    pub fn input_delete_char(&mut self) {
        let cursor_is_not_leftmost = self.user_input_info.character_index != 0;

        if cursor_is_not_leftmost {
            let current_index = self.user_input_info.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.user_input_info.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.user_input_info.input.chars().skip(current_index);

            self.user_input_info.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.input_move_cursor_left();
        }
    }

    fn input_byte_index(&self) -> usize {
        self.user_input_info.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.user_input_info.character_index)
            .unwrap_or(self.user_input_info.input.len())
    }

    pub fn input_submit_number(&mut self) {
        if self.user_input_info.input.is_empty() {
            return;
        }

        self.user_input_history.push(GameMove {
            user_value: self.user_input_info.input.clone().parse().unwrap(),
            move_done_at: Local::now(),
        });

        self.user_input_info.input.clear();
        self.input_reset_cursor();
    }

    pub fn input_submit_name(&mut self) {
        if self.user_input_info.input.is_empty() {
            return;
        }

        // save name logic here

        self.user_input_info.input.clear();
        self.input_reset_cursor();
    }
}
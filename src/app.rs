use std::cmp::{Ordering, Reverse};
use chrono::{Local};
use rand::Rng;
use ratatui::widgets::ListState;
use crate::models::current_screen::CurrentScreen;
use crate::models::game_info::GameInfo;
use crate::models::game_move::GameMove;
use crate::models::user_input_info::UserInputInfo;
use crate::models::user_input_mode::UserInputMode;
use crate::scores::add_score;

pub struct App {
    pub current_screen: CurrentScreen,
    pub main_menu_item_selected: ListState,
    pub main_menu_items: Vec<String>,
    pub game_info: GameInfo,
    pub user_input_history: Vec<GameMove>,
    pub quit_confirm_popup: bool,
    pub user_input_info: UserInputInfo,
    pub user_name: String,
    pub mode: UserInputMode,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
            quit_confirm_popup: false,
            user_input_history: Vec::<GameMove>::new(),
            user_name: String::new(),
            game_info: GameInfo {
                min_number: 0,
                max_number: 0,
                generated_number: 0,
                current_guess_response: String::new(),
                is_game_over: false,
                game_started_at: Local::now(),
                game_completed_at: Local::now(),
                is_hard_mode: false,
            },
            main_menu_item_selected: ListState::default().with_selected(Some(0)),
            main_menu_items: vec![
                "Start game (1-100)".to_string(),
                "Start game (1-100) Hard Mode".to_string(),
                "Start game (1-1000)".to_string(),
                "Start game (1-1000) Hard Mode".to_string(),
                "Start game (1-1000000) Hard Mode".to_string(),
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

    pub fn start_game(&mut self, min_number: i32, max_number: i32, is_hard_mode: bool) {
        self.game_info.generated_number = rand::thread_rng()
            .gen_range(min_number..=max_number);

        self.user_input_info.input.clear();
        self.input_reset_cursor();

        self.user_input_history.clear();

        self.game_info.game_started_at = Local::now();
        self.game_info.is_game_over = false;
        self.game_info.current_guess_response = String::new();
        self.game_info.min_number = min_number;
        self.game_info.max_number = max_number;
        self.game_info.is_hard_mode = is_hard_mode;
        self.mode = UserInputMode::InputNumber;
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

        self.user_input_history.sort_by_key(|x| Reverse(x.move_done_at));

        self.user_input_info.input.clear();
        self.input_reset_cursor();

        self.check_entered_guess();
    }

    fn check_entered_guess(&mut self) {
        let value = self.user_input_history.first().unwrap().user_value;
        match value.cmp(&self.game_info.generated_number) {
            Ordering::Less => {
                self.game_info.current_guess_response = format!("Number is > than {value}");
            },
            Ordering::Greater => {
                self.game_info.current_guess_response = format!("Number is < than {value}");
            },
            Ordering::Equal => {
                self.game_info.current_guess_response = "YOU WON !!!".to_owned();
                self.game_info.is_game_over = true;
                self.game_info.game_completed_at = Local::now();
                self.mode = UserInputMode::InputName;
            },
        }
    }

    pub fn input_submit_name(&mut self) {
        if self.user_input_info.input.is_empty() {
            return;
        }

        self.user_name = self.user_input_info.input.clone();

        self.user_input_info.input.clear();
        self.input_reset_cursor();

        self.remap_scores_and_save();
        self.current_screen = CurrentScreen::Leaderboard;
    }

    fn remap_scores_and_save(&self) {
        let tries = self.user_input_history.len() as i32;
        let number_range = format!("{}-{}", self.game_info.min_number, self.game_info.max_number);
        let time_diff = self.game_info.game_completed_at.time() - self.game_info.game_started_at.time();
        let ms_diff = time_diff.num_milliseconds();

        add_score(self.user_name.clone(),
          tries, number_range,
          self.game_info.game_started_at,
          self.game_info.game_completed_at,
          ms_diff,
          self.game_info.is_hard_mode
        );
    }
}
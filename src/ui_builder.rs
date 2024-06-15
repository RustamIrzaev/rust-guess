use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph},
};

use crate::app::{App};
use crate::models::current_screen::CurrentScreen;
use crate::scores::load_scores;
use crate::ui::ui_footer::{create_footer_left_part, create_footer_navigation};
use crate::ui::ui_header::create_header;
use crate::ui::ui_helpers::centered_rect;
use crate::ui::ui_leaderboard::render_leaderboard_table;

const INFO_TEXT: &str = "(↑) move up | (↓) move down | (Enter) select";

pub fn ui_builder(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([
        Constraint::Length(3),
        Constraint::Max(1),
        Constraint::Min(1),
        Constraint::Length(3),
    ]).split(f.size());

    match app.current_screen {
        CurrentScreen::Menu => {
            f.render_widget({
                Paragraph::new(Line::from("MAIN MENU"))
                    .white().on_dark_gray()
                    .centered()
                    .block(Block::bordered()
                               .border_type(BorderType::Thick)
                               .border_style(Style::new().fg(Color::DarkGray)),
                    )
            }, rects[0]);

            f.render_widget({
                Paragraph::new(Line::from("Select an option").dark_gray())
            }, rects[1]);

            let mut menu_items = Vec::<ListItem>::new();

            for item in app.main_menu_items.iter() {
                menu_items.push(ListItem::new(Text::from(item.to_owned())).white());
            }

            f.render_stateful_widget({
                 List::new(menu_items)
                     .block(Block::default().borders(Borders::ALL))
                     .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                     .highlight_symbol(">> ")
            }, rects[2], &mut app.main_menu_item_selected);

            f.render_widget({
                Paragraph::new(Line::from(INFO_TEXT))
                    .white().on_dark_gray()
                    .centered()
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Thick)
                            .border_style(Style::new().fg(Color::DarkGray)),
                    )
            }, rects[3]);
        },
        CurrentScreen::Leaderboard => {
            f.render_widget(create_header("Leaderboard"), rects[0]);

            let footer_rects = Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(rects[3]);

            let scores = load_scores();

            render_leaderboard_table(f, rects[2], &scores);

            f.render_widget(create_footer_left_part(&app), footer_rects[0]);
            f.render_widget(create_footer_navigation("(q) to back to menu"), footer_rects[1]);
        },
        CurrentScreen::Game => {
            let default_header = format!("Guess the number {}-{}{}!", app.game_info.min_number, app.game_info.max_number,
                 if app.game_info.is_hard_mode { " [H]" } else { "" });

            f.render_widget(create_header(match app.game_info.current_guess_response.len() {
                0 => default_header.as_str(),
                _ => app.game_info.current_guess_response.as_str(),
            }), rects[0]);

            let guess_choice = format!("Enter your guess: {}", app.user_input_info.input);
            let name_choice = format!("Enter your name: {}", app.user_input_info.input);

            f.render_widget({
                let text = match app.game_info.is_game_over {
                    true => name_choice.as_str(),
                    false => guess_choice.as_str(),
                };

                Span::from(text)
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightGreen)
            }, rects[1]);

            if !app.game_info.is_hard_mode {
                f.render_widget({
                    let mut list_items = Vec::<ListItem>::new();

                    for item in app.user_input_history.iter() {
                        list_items.push(ListItem::new(Line::from(Span::styled(
                            format!("{}", item.user_value),
                            Style::default().fg(Color::DarkGray),
                        ))));
                    }

                    List::new(list_items)
                }, rects[2]);
            }

            let footer_rects = Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(rects[3]);

            f.render_widget(create_footer_left_part(&app), footer_rects[0]);
            f.render_widget(create_footer_navigation("(q) to end game"), footer_rects[1]);

            if app.quit_confirm_popup {
                let block = Block::bordered()
                    .title_top(Line::from("Confirmation").centered().on_light_red().white())
                    .light_red();

                let area = centered_rect(60, 16, f.size());
                let text_area = block.inner(area);
                let content = Paragraph::new("Quit the game (y/n)?")
                    .centered()
                    .white();

                f.render_widget(Clear, area);
                f.render_widget(block, area);
                f.render_widget(content, text_area);
            }
        }
    }
}
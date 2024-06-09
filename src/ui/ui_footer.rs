use ratatui::{
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color, Stylize},
    text::{Span, Line},
};
use crate::app::{App};
use crate::models::CurrentScreen;

pub fn create_footer_navigation<'a>(text: &str) -> Paragraph<'a> {
    let footer_hotkeys_data = Span::styled(text.to_owned(), 
       Style::default().fg(Color::LightRed)).into_centered_line();

    Paragraph::new(
        Line::from(footer_hotkeys_data))
        .block(Block::default().borders(Borders::ALL))
}

pub fn create_footer_left_part<'a>(app: &App) -> Paragraph<'a> {
    let footer_text_data = match app.current_screen {
        CurrentScreen::Game => {
            let guesses_made: i32 = app.user_input_history.len() as i32;
            let guess_color = match guesses_made {
                0..=3 => Color::LightGreen,
                4..=7 => Color::Yellow,
                8..=11 => Color::LightRed,
                _ => Color::Red,
            };
            let span_style = Style::default().fg(guess_color);

            vec![
                Span::styled("guesses made", span_style),
                Span::styled(" : ", span_style),
                Span::styled(guesses_made.to_string(), span_style),
            ]},
        CurrentScreen::Leaderboard => {
            vec![Span::from("Top 15 shown").light_green()]
        },
        _ => {vec![]}
    };

    let footer_text = if footer_text_data.len() > 0 {
        Paragraph::new(
            Line::from(footer_text_data))
            .block(Block::default().borders(Borders::ALL))
            .centered()
    } else {
        Paragraph::new(Line::from(""))
    };

    return footer_text;
}
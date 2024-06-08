use ratatui:: {
    layout::{Layout, Constraint, Rect},
    widgets::{Block, Borders, Paragraph, BorderType, Clear,
              List, ListItem, Cell, Row, Table, HighlightSpacing},
    style::{Style, Color, Modifier, Stylize,
            palette::{tailwind}},
    text::{Text, Span, Line},
    Frame,
};
use crate::app::{App, CurrentScreen, NUM_MAXIMUM, NUM_MINIMUM};
use crate::scores::{load_scores, Score};

const INFO_TEXT: &str = "(↑) move up | (↓) move down | (Enter) select";

pub fn ui(f: &mut Frame, app: &mut App) {
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
            let default_header = format!("Guess the number {}-{}!", NUM_MINIMUM, NUM_MAXIMUM);

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

fn create_header<'a>(title_text: &str) -> Paragraph<'a> {
    let title = Paragraph::new(Text::from(title_text.to_owned()).green())
        .block(Block::default().borders(Borders::ALL))
        .centered();

    return title;
}

fn create_footer_left_part<'a>(app: &App) -> Paragraph<'a> {
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

fn create_footer_navigation<'a>(text: &str) -> Paragraph<'a> {
    let footer_hotkeys_data = Span::styled(text.to_owned(),
       Style::default().fg(Color::LightRed)).into_centered_line();

    let footer_hotkeys = Paragraph::new(
        Line::from(footer_hotkeys_data))
        .block(Block::default().borders(Borders::ALL));

    return footer_hotkeys;
}

fn render_leaderboard_table(f: &mut Frame, area: Rect, scores: &Vec<Score>) {
    let header_style = Style::default()
        .fg(tailwind::SLATE.c200)
        .bg(tailwind::BLUE.c900);

    let header = ["#", "Name", "Guess tries", "Game completion time"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows = scores.iter().enumerate().take(15).map(|(i, data)| {
        let color = match i % 2 {
            0 => tailwind::SLATE.c950,
            _ => tailwind::SLATE.c900,
        };

        Row::new(vec![
            Cell::from(Text::from(format!("{}", i + 1)))
                .style(Style::new().fg(tailwind::SLATE.c600).bg(color)),
            Cell::from(Text::from(format!("{}", data.name)))
                .style(Style::new().fg(tailwind::SLATE.c200).bg(color)),
            Cell::from(Text::from(format!("{}", data.tries))
                .centered())
                .style(Style::new().fg(tailwind::GREEN.c300).bg(color)),
            Cell::from(Text::from(format!("{}ms", data.completed_for_ms)))
                .style(Style::new().fg(tailwind::SLATE.c600).bg(color)),
        ])
    });

    let bar = " █ ";
    let longest_score_item_len = constraint_len_calculator(&scores);

    let table = Table::new(rows,
                           [
            // Constraint::Length(longest_score_item_len.0 + 1),
            Constraint::Length(2),
            Constraint::Min(longest_score_item_len.0 + 1),
            Constraint::Min(longest_score_item_len.1 + 1),
            Constraint::Min(longest_score_item_len.2),
        ],
    )
        .header(header)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(tailwind::SLATE.c950)
        .highlight_spacing(HighlightSpacing::Always);
    f.render_widget(table, area);
}

fn constraint_len_calculator(score: &[Score]) -> (u16, u16, u16) {
    let name_len = score
        .iter()
        .map(|x| x.name.as_str().len())
        .max()
        .unwrap_or(0);

    let tries_len = score
        .iter()
        .map(|q| { q.tries.to_string().len()})
        .max()
        .unwrap_or(0);

    let completed_for_msec_len = score
        .iter()
        .map(|q| { q.completed_for_ms.to_string().len()})
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (name_len as u16, tries_len as u16, completed_for_msec_len as u16)
}

fn centered_rect(x_percent: u16, y_percent: u16, rect: Rect) -> Rect {
    let layout = Layout::vertical([
        Constraint::Percentage((100 - y_percent) / 2),
        Constraint::Percentage(y_percent),
        Constraint::Percentage((100 - y_percent) / 2),
    ])
    .split(rect);

    Layout::horizontal([
        Constraint::Percentage((100 - x_percent) / 2),
        Constraint::Percentage(x_percent),
        Constraint::Percentage((100 - x_percent) / 2),
    ])
    .split(layout[1])[1]
}
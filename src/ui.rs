use ratatui:: {
    layout::{Layout, Direction, Constraint},
    widgets::{Block, Borders, Paragraph, BorderType, List, ListItem},
    style::{Style, Color},
    text::{Text, Span, Line},
    Frame,
};
use ratatui::style::Modifier;
use crate::app::{App, CurrentScreen};

const INFO_TEXT: &str = "(↑) move up | (↓) move down | (Enter) select";

pub fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ]).split(f.size());

    match app.current_screen {
        CurrentScreen::Menu => {
            let title = Paragraph::new(Line::from("MAIN MENU"))
                .style(Style::new().fg(Color::White).bg(Color::DarkGray))
                .centered()
                .block(
                    Block::bordered()
                        .border_type(BorderType::Thick)
                        .border_style(Style::new().fg(Color::DarkGray)),
                );

            f.render_widget(title, rects[0]);

            let mut menu_items = Vec::<ListItem>::new();

            for item in app.main_menu_items.iter() {
                menu_items.push(ListItem::new(Span::styled(
                    item,
                    Style::default().fg(Color::White),
                )));
            }

            let list = List::new(menu_items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, rects[1], &mut app.main_menu_item_selected);

            let info_footer = Paragraph::new(Line::from(INFO_TEXT))
                .style(Style::new().fg(Color::White).bg(Color::DarkGray))
                .centered()
                .block(
                    Block::bordered()
                        .border_type(BorderType::Thick)
                        .border_style(Style::new().fg(Color::DarkGray)),
                );

            f.render_widget(info_footer, rects[2]);
        },
        CurrentScreen::Leaderboard => {
            f.render_widget(create_header("Leaderboard"), rects[0]);

            let footer_rects = Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(rects[2]);

            f.render_widget(create_footer_left_part(&app), footer_rects[0]);
            f.render_widget(create_footer_navigation("(q) to back to menu"), footer_rects[1]);
        },
        CurrentScreen::Quit => {
            f.render_widget(create_header("End the game? (y/n)"), rects[0]);
        },
        _ => {
            f.render_widget(create_header("Guess the number!"), rects[0]);

            let list = create_main_block(&app);
            f.render_widget(list, rects[1]);

            let footer_rects = Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(rects[2]);

            f.render_widget(create_footer_left_part(&app), footer_rects[0]);
            f.render_widget(create_footer_navigation("(q) to end game"), footer_rects[1]);
        }
    }
}

fn create_main_block<'a>(app: &App) -> List<'a> {
    let mut list_items = Vec::<ListItem>::new();

    for item in app.user_input_history.iter() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", item.user_value),
            Style::default().fg(Color::Gray),
        ))));
    }

    let list = List::new(list_items);

    return list;
}

fn create_header<'a>(title_text: &str) -> Paragraph<'a> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        title_text.to_owned(),
        Style::default().fg(Color::Green),
    ))
        .block(title_block).centered();

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
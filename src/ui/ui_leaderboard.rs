use ratatui:: {
    layout::{Constraint, Rect},
    widgets::{Cell, Row, Table, HighlightSpacing},
    style::{Style, palette::tailwind, Stylize},
    text::{Text},
    Frame,
};
use crate::scores::Score;
use crate::ui::ui_helpers::constraint_len_calculator;

pub fn render_leaderboard_table(f: &mut Frame, area: Rect, scores: &Vec<Score>) {
    let header_style = Style::default()
        .fg(tailwind::SLATE.c200)
        .bg(tailwind::BLUE.c900);

    let header = ["#", "Name", "Tries", "Game range", "Mode", "Game time"]
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
            Cell::from(Text::from(format!("{}", data.number_range))
                .centered())
                .style(Style::new().fg(tailwind::SLATE.c200).bg(color)),
            Cell::from(Text::from(format!("{}", if data.is_hard_mode {"H"} else {""}))
                .centered())
                .style(Style::new().fg(tailwind::SLATE.c200).bg(color)),
            Cell::from(Text::from(format!("{}ms", data.completed_for_ms))
                .centered())
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
                               Constraint::Min(longest_score_item_len.1 + 2),
                               Constraint::Min(longest_score_item_len.1 + 3),
                               Constraint::Min(longest_score_item_len.4),
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
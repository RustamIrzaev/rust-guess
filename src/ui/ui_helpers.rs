use ratatui::layout::{Constraint, Layout, Rect};
use crate::models::Score;

pub fn constraint_len_calculator(score: &[Score]) -> (u16, u16, u16, u16, u16) {
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

    let number_range_len = score
        .iter()
        .map(|q| { q.number_range.as_str().len()})
        .max()
        .unwrap_or(0);

    let completed_for_msec_len = score
        .iter()
        .map(|q| { q.completed_for_ms.to_string().len()})
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (name_len as u16, tries_len as u16, number_range_len as u16, 1, completed_for_msec_len as u16)
}

pub fn centered_rect(x_percent: u16, y_percent: u16, rect: Rect) -> Rect {
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
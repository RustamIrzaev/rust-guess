use ratatui:: {
  widgets::{Block, Borders, Paragraph},
  style::Stylize,
  text::Text,
};

pub fn create_header<'a>(title_text: &str) -> Paragraph<'a> {
    Paragraph::new(Text::from(title_text.to_owned()).green())
        .block(Block::default().borders(Borders::ALL))
        .centered()
}
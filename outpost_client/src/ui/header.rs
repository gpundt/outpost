use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub struct Title<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

pub fn generate_title<'a>(title: &'a Title<'a>) -> Paragraph<'a> {
    let mut spans = Vec::new();

    spans.push(Span::styled(
        title.name,
        Style::default().fg(Color::Magenta),
    ));
    spans.push(Span::styled("  |  ", Style::default().fg(Color::DarkGray)));
    spans.push(Span::styled(
        format!(" {}", title.description),
        Style::default().fg(Color::White),
    ));

    let header_line = Line::from(spans).left_aligned();

    let header_widget = Paragraph::new(header_line).block(
        Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(5, 0, 0, 0)),
    );
    // Wrap a paragraph/line widget and render
    return header_widget;
}

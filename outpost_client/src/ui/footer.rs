use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub struct Keybind {
    pub key: String,
    pub description: String,
}
impl Keybind {
    pub fn new(key: String, description: String) -> Self {
        Self {
            key: key,
            description: description,
        }
    }
}

pub fn generate_keybinds<'a>(keybinds: Vec<Keybind>) -> Paragraph<'a> {
    let mut spans = Vec::new();

    for (i, keybind) in keybinds.iter().enumerate() {
        if i > 0 {
            // Add divider between keybinds
            spans.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
        }

        // Highlught the key in yellow and the description in white
        spans.push(Span::styled(
            keybind.key.clone(),
            Style::default().fg(Color::Magenta),
        ));
        spans.push(Span::styled(
            format!(" {}", keybind.description),
            Style::default().fg(Color::White),
        ));
    }
    let keybind_line = Line::from(spans).left_aligned();

    let footer_widget = Paragraph::new(keybind_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Keybinds")
            .padding(Padding::new(5, 0, 0, 0)),
    );

    return footer_widget;
}
pub enum Severity {
    Successful,
    Info,
    Warning,
    Error,
    Debug,
}

pub struct ClientStatus {
    pub severity: Severity,
    pub message: String,
}
impl ClientStatus {
    pub fn new(severity: Severity, message: String) -> Self {
        Self { severity, message }
    }
}

pub fn generate_client_status<'a>(status: &ClientStatus) -> Paragraph<'a> {
    let mut spans = Vec::new();

    let (mut spans, widget_color) = match status.severity {
        Severity::Info => {
            let info_header = match status.message.as_str() {
                "" => "",
                _ => "Info",
            };
            spans.push(Span::styled(
                format!("{:<10}", info_header),
                Style::default().fg(Color::Blue),
            ));
            (spans, Color::Blue)
        }
        Severity::Error => {
            spans.push(Span::styled(
                format!("{:<10}", "Error"),
                Style::default().fg(Color::Red),
            ));
            (spans, Color::Red)
        }
        Severity::Warning => {
            spans.push(Span::styled(
                format!("{:<10}", "Warning"),
                Style::default().fg(Color::Yellow),
            ));
            (spans, Color::Yellow)
        }
        Severity::Debug => {
            spans.push(Span::styled(
                format!("{:<10}", "Debug"),
                Style::default().fg(Color::Gray),
            ));
            (spans, Color::Gray)
        }
        Severity::Successful => {
            let info_header = match status.message.as_str() {
                "" => "",
                _ => "Info",
            };
            spans.push(Span::styled(
                format!("{:<10}", info_header),
                Style::default().fg(Color::Green),
            ));
            (spans, Color::Green)
        }
    };

    spans.push(Span::styled(
        status.message.to_string(),
        Style::default().fg(Color::White),
    ));
    let status_line = Line::from(spans).left_aligned();
    let status_widget = Paragraph::new(status_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(widget_color))
            .border_type(BorderType::Rounded)
            .title("Status")
            .padding(Padding::new(5, 0, 0, 0)),
    );

    return status_widget;
}

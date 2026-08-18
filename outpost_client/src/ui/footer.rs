use crate::client_http::query::query_server_status;
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
    pub fn new() -> Self {
        Self {
            severity: Severity::Info,
            message: "".to_string(),
        }
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

#[derive(Clone, Default)]
pub struct ServerStatusCache {
    pub server_connection: bool,
    pub database_connection: bool,
    pub serial_connection: bool,
}

pub async fn fetch_server_status() -> ServerStatusCache {
    match query_server_status().await {
        Some(s) => ServerStatusCache {
            server_connection: true,
            database_connection: s.database_reachable,
            serial_connection: s.serial_connected,
        },
        None => ServerStatusCache::default(),
    }
}

pub fn generate_server_status<'a>(status: &ServerStatusCache) -> Paragraph<'a> {
    let mut spans = Vec::new();
    spans.push(Span::styled(
        format!(
            "Server: {}",
            if status.server_connection {
                "\x1b[32m\u{2713}\x1b[0m"
            } else {
                "\x1b[31m\u{2717}\x1b[0m"
            }
        ),
        Style::default().fg(Color::White),
    ));
    spans.push(Span::styled("  |  ", Style::default().fg(Color::DarkGray)));

    spans.push(Span::styled(
        format!(
            "Database: {}",
            if status.database_connection {
                "\x1b[32m\u{2713}\x1b[0m"
            } else {
                "\x1b[31m\u{2717}\x1b[0m"
            }
        ),
        Style::default().fg(Color::White),
    ));
    spans.push(Span::styled("  |  ", Style::default().fg(Color::DarkGray)));

    spans.push(Span::styled(
        format!(
            "Meshtastic: {}",
            if status.serial_connection {
                "\x1b[32m\u{2713}\x1b[0m"
            } else {
                "\x1b[31m\u{2717}\x1b[0m"
            }
        ),
        Style::default().fg(Color::White),
    ));

    let status_line = Line::from(spans).left_aligned();
    Paragraph::new(status_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Status")
            .padding(Padding::new(5, 0, 0, 0)),
    )
}

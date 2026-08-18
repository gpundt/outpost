use std::sync::{Arc, OnceLock, RwLock};

use crate::{
    client_http::query::{QueryResponse::Status, query_server_status},
    ui::footer::{ClientStatus, Severity},
};
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

#[derive(Clone, Default)]
pub struct ServerStatusCache {
    pub server_connection: bool,
    pub database_connection: bool,
    pub serial_connection: bool,
}

pub fn global_server_status() -> &'static Arc<RwLock<ServerStatusCache>> {
    static STATUS: OnceLock<Arc<RwLock<ServerStatusCache>>> = OnceLock::new();
    STATUS.get_or_init(|| Arc::new(RwLock::new(ServerStatusCache::default())))
}

pub async fn update_server_status() {
    match query_server_status().await {
        Some(s) => {
            let mut status = global_server_status().write().unwrap();
            status.server_connection = true;
            status.database_connection = s.database_reachable;
            status.serial_connection = s.serial_connected;
        }
        None => {
            let mut status = global_server_status().write().unwrap();
            status.server_connection = false;
            status.database_connection = false;
            status.serial_connection = false;
        }
    }
}

pub fn generate_server_status<'a>() -> (Paragraph<'a>, Paragraph<'a>, Paragraph<'a>) {
    let server_status = global_server_status().read().unwrap();
    let mut server_connection_span = Vec::new();
    server_connection_span.push(
        Span::raw(if server_status.server_connection {
            "\u{2713}"
        } else {
            "\u{2717}"
        })
        .style(if server_status.server_connection {
            Color::Green
        } else {
            Color::Red
        }),
    );
    let server_connection_line = Line::from(server_connection_span).centered();
    let server_connection_paragraph = Paragraph::new(server_connection_line).block(
        Block::default()
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Server"),
    );

    let mut database_connection_span = Vec::new();
    database_connection_span.push(
        Span::raw(if server_status.database_connection {
            "\u{2713}"
        } else {
            "\u{2717}"
        })
        .style(if server_status.database_connection {
            Color::Green
        } else {
            Color::Red
        }),
    );
    let database_connection_line = Line::from(database_connection_span).centered();
    let database_connection_paragraph = Paragraph::new(database_connection_line).block(
        Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Database"),
    );

    let mut serial_connection_span = Vec::new();
    serial_connection_span.push(
        Span::raw(if server_status.serial_connection {
            "\u{2713}"
        } else {
            "\u{2717}"
        })
        .style(if server_status.serial_connection {
            Color::Green
        } else {
            Color::Red
        }),
    );
    let serial_connection_line = Line::from(serial_connection_span).centered();
    let serial_connection_paragraph = Paragraph::new(serial_connection_line).block(
        Block::default()
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Serial"),
    );

    (
        server_connection_paragraph,
        database_connection_paragraph,
        serial_connection_paragraph,
    )
}

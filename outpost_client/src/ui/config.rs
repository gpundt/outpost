use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{arguments::get_arguments, client_http::storage::get_server_config};

pub fn generate_server_url_widget<'a>() -> Paragraph<'a> {
    let mut spans = Vec::new();

    let server_config = get_server_config();
    let server_ip = get_arguments().server_ip.clone();
    match server_config {
        Some(config) => {
            spans.push(Span::styled(
                format!("http://{}:{}", server_ip, config.http_port),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            ));
        }
        None => {
            spans.push(Span::styled(
                format!("No server config received..."),
                Style::default().fg(Color::White),
            ));
        }
    }

    let server_url_line = Line::from(spans).centered();

    return Paragraph::new(server_url_line).block(
        Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Server URL"),
    );
}

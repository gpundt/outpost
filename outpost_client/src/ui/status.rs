use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::client_http::storage::get_server_status;

pub fn generate_server_uptime_widget<'a>() -> Paragraph<'a> {
    let mut spans = Vec::new();

    let server_status = get_server_status();

    match server_status {
        Some(status) => {
            spans.push(Span::styled(
                format!("{}", status.uptime),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            ));
        }
        None => {
            spans.push(Span::styled(
                format!("No server status received..."),
                Style::default().fg(Color::White),
            ));
        }
    }

    let uptime_line = Line::from(spans).centered();

    let uptime_widget = Paragraph::new(uptime_line).block(
        Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Uptime"),
    );
    return uptime_widget;
}

pub fn generate_server_version_widget<'a>() -> Paragraph<'a> {
    let mut spans = Vec::new();

    let server_status = get_server_status();

    match server_status {
        Some(status) => {
            spans.push(Span::styled(
                format!("{}", status.version),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            ));
        }
        None => {
            spans.push(Span::styled(
                format!("Unreachable"),
                Style::default().fg(Color::Red),
            ));
        }
    }

    let version_line = Line::from(spans).centered();

    let version_widget = Paragraph::new(version_line).block(
        Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Version"),
    );
    return version_widget;
}

pub fn generate_server_serial_port_widget<'a>() -> Paragraph<'a> {
    let mut spans = Vec::new();

    let server_status = get_server_status();

    match server_status {
        Some(status) => {
            spans.push(Span::styled(
                format!("{}", status.serial_port),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            ));
        }
        None => {
            spans.push(Span::styled(
                format!("Unreachable"),
                Style::default().fg(Color::Red),
            ));
        }
    }

    let serial_port_line = Line::from(spans).centered();

    let serial_port_widget = Paragraph::new(serial_port_line).block(
        Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Serial Port"),
    );
    return serial_port_widget;
}

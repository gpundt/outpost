use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use std::{io, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use super::{
    footer::{ClientStatus, Keybind, Severity, generate_client_status, generate_keybinds},
    frame::{FrameMode, NextFrame},
    header::{Title, generate_server_status, generate_title},
};

use crossterm::event;

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

pub fn generate_server_status_widget<'a>() -> Paragraph<'a> {
    let mut lines = Vec::new();

    let server_status = get_server_status();
    match server_status {
        Some(status) => {
            // Server Status
            let server_status_key = Span::styled(
                format!("  {:<20}:", "Status".to_string()),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            );
            let server_status_value = Span::styled(
                format!("  {}", status.status),
                Style::default().fg(Color::White),
            );
            lines.push(Line::from(vec![server_status_key, server_status_value]));

            // Server Uptime
            let server_uptime_key = Span::styled(
                format!("  {:<20}:", "Uptime".to_string()),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            );
            let server_uptime_value = Span::styled(
                format!("  {}", status.uptime),
                Style::default().fg(Color::White),
            );
            lines.push(Line::from(vec![server_uptime_key, server_uptime_value]));

            // Server Version
            let server_version_key = Span::styled(
                format!("  {:<20}:", "Version".to_string()),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            );
            let server_version_value = Span::styled(
                format!("  {}", status.version),
                Style::default().fg(Color::White),
            );
            lines.push(Line::from(vec![server_version_key, server_version_value]));

            // Serial Connected
            let serial_connected_key = Span::styled(
                format!("  {:<20}:", "Serial Connected".to_string()),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            );
            let serial_connected_value = Span::styled(
                format!("  {}", status.serial_connected),
                if status.serial_connected {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                },
            );
            lines.push(Line::from(vec![
                serial_connected_key,
                serial_connected_value,
            ]));

            // Serial Port
            let serial_port_key = Span::styled(
                format!("  {:<20}:", "Serial Port".to_string()),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            );
            let serial_port_value = Span::styled(
                format!("  {}", status.serial_port),
                Style::default().fg(Color::White),
            );
            lines.push(Line::from(vec![serial_port_key, serial_port_value]));

            // Database reachable
            let database_reachable_key = Span::styled(
                format!("  {:<20}:", "Database Reachable".to_string()),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            );
            let database_reachable_value = Span::styled(
                format!("  {}", status.database_reachable),
                if status.database_reachable {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                },
            );
            lines.push(Line::from(vec![
                database_reachable_key,
                database_reachable_value,
            ]));
        }
        None => {
            lines.push(Line::from(Span::styled(
                format!("No server status received..."),
                Style::default().fg(Color::White),
            )));
        }
    }

    return Paragraph::new(lines).block(
        Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Server Status"),
    );
}

pub struct StatusFrame {
    /// Specifies which mode we're on
    pub mode: FrameMode,
    /// Current status
    pub status: ClientStatus,
    /// Next frame to render
    pub next_frame: NextFrame,
    /// Current status widget scroll offset
    pub current_status_offset: u16,
}
/// Functions that can be implemented by StatusFrame
impl StatusFrame {
    /// Function to create a new status frame object
    pub fn new() -> StatusFrame {
        StatusFrame {
            mode: FrameMode::default(),
            status: ClientStatus {
                severity: Severity::Info,
                message: "".to_string(),
            },
            next_frame: NextFrame::Status,
            current_status_offset: 0,
        }
    }

    /// Renders the current frame in a loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<NextFrame> {
        loop {
            // Pull the latest value out of the channel - non-blocking
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(Duration::from_millis(500))? {
                self.handle_events()?
            }

            match self.mode {
                FrameMode::Exit => return Ok(NextFrame::Dashboard),
                FrameMode::Navigation => {}
                FrameMode::ChangeFrame => return Ok(self.next_frame.clone()),
            }
        }
    }

    /// Function to draw all dashboard widgets to the terminal
    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Header
        let header_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(75),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(chunks[0]);
        frame.render_widget(
            generate_title(&Title {
                name: "Outpost-Server Status",
                description: "View Outpost-Server Status Messages",
            }),
            header_row[0],
        );

        let (
            server_connection_paragraph,
            database_connection_paragraph,
            serial_connection_paragraph,
        ) = generate_server_status();

        frame.render_widget(server_connection_paragraph, header_row[1]);
        frame.render_widget(database_connection_paragraph, header_row[2]);
        frame.render_widget(serial_connection_paragraph, header_row[3]);

        // Main Content
        frame.render_widget(generate_server_status_widget(), chunks[1]);

        // Footer
        let footer_content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(50), Constraint::Min(50)])
            .split(chunks[2]);
        // Keybinds
        let mut keybinds: Vec<Keybind> = Vec::new();
        keybinds.push(Keybind::new("q / Esc".to_string(), "Dashboard".to_string()));

        frame.render_widget(generate_keybinds(keybinds), footer_content[0]);
        // Status
        frame.render_widget(generate_client_status(&self.status), footer_content[1]);
    }
    /// Function to handle user input events
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }

            _ => {}
        };
        Ok(())
    }

    /// Function to handle user key input events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.mode = FrameMode::Exit;
            }
            _ => {}
        }
    }

    /// Function to safely update self.status
    fn update_status(&mut self, client_status: ClientStatus) {
        self.status = client_status;
    }
}

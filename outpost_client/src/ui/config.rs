use std::{io, time::Duration};

use crossterm::event;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use super::{
    footer::{ClientStatus, Keybind, Severity, generate_client_status, generate_keybinds},
    frame::{FrameMode, NextFrame},
    header::{Title, generate_server_status, generate_title},
};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

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

pub struct ConfigFrame {
    /// Specifies which mode we're on
    pub mode: FrameMode,
    /// Current status
    pub status: ClientStatus,
    /// Next frame to render
    pub next_frame: NextFrame,
    /// Current config widget scroll offset
    pub current_config_offset: u16,
}
/// Functions that can be implemented by ConfigFrame
impl ConfigFrame {
    /// Function to create a new config frame object
    pub fn new() -> ConfigFrame {
        ConfigFrame {
            mode: FrameMode::default(),
            status: ClientStatus {
                severity: Severity::Info,
                message: "".to_string(),
            },
            next_frame: NextFrame::Config,
            current_config_offset: 0,
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
                FrameMode::Navigation => return Ok(self.next_frame.clone()),
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
                name: "Outpost-Server Configuration",
                description: "View Outpost-Server Configration Values",
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

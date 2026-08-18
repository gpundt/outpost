use std::{io, time::Duration};

use crate::ui::header::global_server_status;

use super::{
    footer::{ClientStatus, Keybind, Severity, generate_client_status, generate_keybinds},
    frame::NextFrame,
    header::{ServerStatusCache, Title, generate_server_status, generate_title},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};
use tokio::sync::watch;

#[derive(Debug, Default)]
pub enum DashboardMode {
    #[default]
    Navigation,
    ListSelected,
    Exit,
}

pub struct Dashboard {
    /// Specifies which mode we're on
    pub mode: DashboardMode,
    /// Current status
    status: ClientStatus,
}

/// Functions that can be imlemented by DeviceDashboard
impl Dashboard {
    /// Function to create a new dashboard object
    pub fn new() -> Self {
        Self {
            mode: DashboardMode::default(),
            status: ClientStatus {
                severity: Severity::Info,
                message: "".to_string(),
            },
        }
    }

    /// Renders the current frame in a loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<NextFrame> {
        loop {
            // Pull the latest value out of the channel — non-blocking, no .await needed.
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(Duration::from_millis(500))? {
                self.handle_events()?;
            }

            match self.mode {
                DashboardMode::Exit => return Ok(NextFrame::Exit),
                DashboardMode::Navigation => {}
                DashboardMode::ListSelected => {}
            }
        }
    }

    /// Function to draw all Dashboard widgets to the terminal
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
                name: "Outpost-Client Dashboard",
                description: "Interact with Outpost-Server",
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

        // Main content
        // TODO

        // Footer
        let footer_content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(50), Constraint::Min(50)])
            .split(chunks[2]);
        // Keybinds
        let mut keybinds = Vec::new();
        keybinds.push(Keybind {
            key: "q / Esc".to_string(),
            description: "Exit".to_string(),
        });
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
                self.mode = DashboardMode::Exit;
            }
            _ => {}
        }
    }

    /// Function to safely update self.status
    fn update_status(&mut self, client_status: ClientStatus) {
        self.status = client_status;
    }
}

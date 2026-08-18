use std::{io, time::Duration};

use crate::ui::footer::ServerStatusCache;

use super::{
    footer::{
        ClientStatus, Keybind, Severity, generate_client_status, generate_keybinds,
        generate_server_status,
    },
    frame::NextFrame,
    header::{Title, generate_title},
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
    server_status_rx: watch::Receiver<ServerStatusCache>,
    server_status: ServerStatusCache,
}

/// Functions that can be imlemented by DeviceDashboard
impl Dashboard {
    /// Function to create a new dashboard object
    pub fn new(server_status_rx: watch::Receiver<ServerStatusCache>) -> Self {
        let server_status = server_status_rx.borrow().clone();
        Self {
            mode: DashboardMode::default(),
            status: ClientStatus {
                severity: Severity::Info,
                message: "".to_string(),
            },
            server_status_rx,
            server_status,
        }
    }

    /// Renders the current frame in a loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<NextFrame> {
        loop {
            // Pull the latest value out of the channel — non-blocking, no .await needed.
            if self.server_status_rx.has_changed().unwrap_or(false) {
                self.server_status = self.server_status_rx.borrow_and_update().clone();
            }

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
    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Header
        frame.render_widget(
            generate_title(&Title {
                name: "Outpost-Client Dashboard",
                description: "Interact with Outpost-Server",
            }),
            chunks[0],
        );

        // Main content

        // Footer
        let server_status_content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(chunks[2]);
        frame.render_widget(
            generate_server_status(&self.server_status),
            server_status_content[0],
        );

        let footer_content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(50), Constraint::Min(50)])
            .split(chunks[3]);
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
}

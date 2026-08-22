use std::{io, time::Duration};

use crate::ui::{
    config::generate_server_url_widget,
    nodes::generate_nodes_dashbaord_widget,
    status::{
        generate_server_serial_port_widget, generate_server_uptime_widget,
        generate_server_version_widget,
    },
    tasks::generate_tasks_dashboard_widget,
    texts::generate_texts_dashboard_widget,
};

use super::{
    footer::{ClientStatus, Keybind, Severity, generate_client_status, generate_keybinds},
    frame::NextFrame,
    header::{Title, generate_server_status, generate_title},
};

use crate::client_http::storage::{get_server_nodes, get_server_tasks, get_server_texts};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};

pub enum DashboardWidgets {
    TEXTS,
    TASKS,
    NODES,
}

#[derive(Debug, Default)]
pub enum DashboardMode {
    #[default]
    Navigation,
    Exit,
}

pub struct Dashboard {
    /// Specifies which mode we're on
    pub mode: DashboardMode,
    /// Current status
    pub status: ClientStatus,
    /// Current Texts widget scroll offset
    pub current_texts_offset: u16,
    /// Current Tasks widget scroll offset
    pub current_tasks_offset: u16,
    /// Current Nodes widget scroll offset
    pub current_nodes_offset: u16,
    /// Current selected widget
    pub current_widget: Option<DashboardWidgets>,
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
            current_nodes_offset: 0,
            current_tasks_offset: 0,
            current_texts_offset: 0,
            current_widget: None,
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
        let dashboard_content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);
        let dashboard_content_left_half = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(dashboard_content[0]);
        frame.render_widget(
            generate_texts_dashboard_widget(
                self.current_texts_offset,
                matches!(self.current_widget, Some(DashboardWidgets::TEXTS)),
            ),
            dashboard_content_left_half[0],
        );
        frame.render_widget(
            generate_tasks_dashboard_widget(
                self.current_tasks_offset,
                matches!(self.current_widget, Some(DashboardWidgets::TASKS)),
            ),
            dashboard_content_left_half[1],
        );

        let dashboard_content_right_half = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(dashboard_content[1]);

        let dashboard_content_right_half_first_quarter = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(dashboard_content_right_half[0]);
        frame.render_widget(
            generate_server_uptime_widget(),
            dashboard_content_right_half_first_quarter[0],
        );
        frame.render_widget(
            generate_server_version_widget(),
            dashboard_content_right_half_first_quarter[1],
        );
        frame.render_widget(
            generate_nodes_dashbaord_widget(
                self.current_nodes_offset,
                matches!(self.current_widget, Some(DashboardWidgets::NODES)),
            ),
            dashboard_content_right_half[2],
        );

        let dashboard_content_right_half_second_quarter = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(dashboard_content_right_half[1]);
        frame.render_widget(
            generate_server_serial_port_widget(),
            dashboard_content_right_half_second_quarter[0],
        );
        frame.render_widget(
            generate_server_url_widget(),
            dashboard_content_right_half_second_quarter[1],
        );

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
        keybinds.push(Keybind {
            key: "t".to_string(),
            description: "Texts".to_string(),
        });
        keybinds.push(Keybind {
            key: "a".to_string(),
            description: "Tasks".to_string(),
        });
        keybinds.push(Keybind {
            key: "n".to_string(),
            description: "Nodes".to_string(),
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
                if self.current_widget.is_some() {
                    self.current_widget = None
                } else {
                    self.mode = DashboardMode::Exit;
                }
            }
            KeyCode::Char('t') => {
                self.current_widget = Some(DashboardWidgets::TEXTS);
            }
            KeyCode::Char('n') => {
                self.current_widget = Some(DashboardWidgets::NODES);
            }
            KeyCode::Char('a') => {
                self.current_widget = Some(DashboardWidgets::TASKS);
            }
            KeyCode::Down => match self.current_widget {
                Some(DashboardWidgets::TASKS) => {
                    if let Some(tasks) = get_server_tasks() {
                        if self.current_tasks_offset < tasks.len() as u16 {
                            self.current_tasks_offset += 1
                        }
                    }
                }
                Some(DashboardWidgets::NODES) => {
                    if let Some(nodes) = get_server_nodes() {
                        if self.current_nodes_offset < nodes.len() as u16 {
                            self.current_nodes_offset += 1
                        }
                    }
                }
                Some(DashboardWidgets::TEXTS) => {
                    if let Some(texts) = get_server_texts() {
                        if self.current_texts_offset < texts.len() as u16 {
                            self.current_texts_offset += 1
                        }
                    }
                }
                None => {}
            },
            KeyCode::Up => match self.current_widget {
                Some(DashboardWidgets::TASKS) => {
                    if self.current_tasks_offset > 0 {
                        self.current_tasks_offset -= 1
                    }
                }
                Some(DashboardWidgets::NODES) => {
                    if self.current_nodes_offset > 0 {
                        self.current_nodes_offset -= 1
                    }
                }
                Some(DashboardWidgets::TEXTS) => {
                    if self.current_texts_offset > 0 {
                        self.current_texts_offset -= 1
                    }
                }
                None => {}
            },

            _ => {}
        }
    }

    /// Function to safely update self.status
    fn update_status(&mut self, client_status: ClientStatus) {
        self.status = client_status;
    }
}

use crate::{
    client_http::storage::get_server_tasks,
    ui::{
        footer::{ClientStatus, Keybind, Severity, generate_client_status, generate_keybinds},
        frame::{FrameMode, NextFrame},
        header::{Title, generate_server_status, generate_title},
    },
};
use config::tasks::OutpostTask;
use core::fmt;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};
use std::{io, time::Duration};

pub fn generate_tasks_dashboard_widget<'a>(
    current_scroll_offset: u16,
    selected_widget: bool,
) -> Paragraph<'a> {
    let mut lines = Vec::new();

    let tasks = get_server_tasks();

    match tasks {
        Some(tasks_vec) => {
            if tasks_vec.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!(" No outpost tasks issued..."),
                    Style::default().fg(Color::Yellow),
                )));
            }
            for task in tasks_vec {
                lines.push(Line::from(Span::styled(
                    format!(
                        " [{}] {:<15} ->  {}",
                        task.requested_at.format("%m/%d %H:%M:%S"),
                        task.task_type,
                        if task.successful {
                            "Finished"
                        } else {
                            "Failed"
                        }
                    ),
                    Style::default().fg(Color::White),
                )));
            }
        }
        None => {
            lines.push(Line::from(Span::styled(
                format!(" Server connection failed..."),
                Style::default().fg(Color::Red),
            )));
        }
    }

    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if selected_widget {
                    Color::Green
                } else {
                    Color::Blue
                }))
                .border_type(BorderType::Rounded)
                .title("Tasks")
                .padding(Padding::new(3, 0, 0, 0)),
        )
        .scroll((current_scroll_offset, 0))
}

pub fn generate_tasks_list_widget<'a>(current_index: u16) -> Paragraph<'a> {
    let mut lines = Vec::new();

    for (index, choice) in vec![
        OutpostTask::Backup,
        OutpostTask::Beacon,
        OutpostTask::PurgeNodes,
        OutpostTask::PurgeRaw,
        OutpostTask::PurgePositions,
        OutpostTask::ReconnectSerial,
        OutpostTask::Restart,
    ]
    .iter()
    .enumerate()
    {
        let cursor = if index == current_index as usize {
            Span::styled(
                " > ".to_string(),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            )
        } else {
            Span::styled("   ".to_string(), Style::default().fg(Color::White))
        };
        let formatted_task_name = choice
            .to_string()
            .split('_')
            .map(|part| {
                let mut c = part.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str() + " ",
                }
            })
            .collect::<String>();

        lines.push(Line::from(vec![
            cursor,
            Span::styled(formatted_task_name, Style::default().fg(Color::White)),
        ]));
    }

    Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Available Tasks")
            .padding(Padding::new(3, 0, 0, 0)),
    )
}
pub enum TaskWidgets {
    LIST,
    CONFIRM,
    HISTORY,
}
impl fmt::Display for TaskWidgets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskWidgets::LIST => write!(f, "List Widget"),
            TaskWidgets::CONFIRM => write!(f, "Confirm Widget"),
            TaskWidgets::HISTORY => write!(f, "History Widget"),
        }
    }
}
pub struct TasksFrame {
    /// Specifies which mode we're on
    pub mode: FrameMode,
    /// current Status
    pub status: ClientStatus,
    /// Current task list index
    pub current_list_index: u16,
    /// Current history widget scroll offset
    pub current_history_offset: u16,
    /// Current widget
    pub current_widget: TaskWidgets,
    /// Next frame to render
    pub next_frame: NextFrame,
    /// Current task that is selected
    pub selected_task: Option<OutpostTask>,
}

/// Functions that can be implemented by TasksFrame
impl TasksFrame {
    /// Function to create a new tasks frame object
    pub fn new() -> TasksFrame {
        TasksFrame {
            mode: FrameMode::Navigation,
            status: ClientStatus::new(Severity::Info, "".to_string()),
            current_list_index: 0,
            current_history_offset: 0,
            current_widget: TaskWidgets::LIST,
            next_frame: NextFrame::Tasks,
            selected_task: None,
        }
    }

    /// Renders the current frame in a loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<NextFrame> {
        loop {
            // Pull latest value out of channel - non-blocking
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(Duration::from_millis(500))? {
                self.handle_events()?;
            }

            match self.mode {
                FrameMode::Exit => return Ok(NextFrame::Dashboard),
                _ => {}
            }
        }
    }

    /// Function to draw all Tasks widgets to the terminal
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
                name: "Outpost-Server Task Submission",
                description: "Submit Tasks to be Executed on Outpost-Server",
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
        let main_body = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);
        let main_body_top_half = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_body[0]);
        frame.render_widget(
            generate_tasks_list_widget(self.current_list_index),
            main_body_top_half[0],
        );

        let main_body_bottom_half = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_body[1]);
        frame.render_widget(
            generate_tasks_dashboard_widget(
                self.current_history_offset,
                matches!(self.current_widget, TaskWidgets::HISTORY),
            ),
            main_body_bottom_half[0],
        );

        // Footer
        let footer_content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(50), Constraint::Min(50)])
            .split(chunks[2]);
        // Keybinds
        let mut keybinds: Vec<Keybind> = Vec::new();
        keybinds.push(Keybind::new("q / Esc".to_string(), "Dashboard".to_string()));
        keybinds.push(Keybind::new("Enter".to_string(), "Select".to_string()));
        keybinds.push(Keybind::new("h".to_string(), "Task History".to_string()));
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
                if matches!(&self.current_widget, TaskWidgets::LIST) {
                    self.mode = FrameMode::Exit;
                } else {
                    self.update_status(ClientStatus::new(
                        Severity::Info,
                        format!("{}: Unselected", &self.current_widget),
                    ));
                    self.current_widget = TaskWidgets::LIST;
                    self.selected_task = None;
                }
            }
            KeyCode::Char('h') => {
                self.current_widget = TaskWidgets::HISTORY;
                self.update_status(ClientStatus::new(
                    Severity::Info,
                    "History Widget: Selected".to_string(),
                ));
            }
            KeyCode::Down => match self.current_widget {
                TaskWidgets::HISTORY => {
                    if let Some(tasks) = get_server_tasks() {
                        if self.current_history_offset < tasks.len() as u16 {
                            self.current_history_offset += 1
                        }
                    }
                }
                TaskWidgets::LIST => {
                    if self.current_list_index
                        < vec![
                            OutpostTask::Backup,
                            OutpostTask::Beacon,
                            OutpostTask::PurgeNodes,
                            OutpostTask::PurgeRaw,
                            OutpostTask::PurgePositions,
                            OutpostTask::ReconnectSerial,
                            OutpostTask::Restart,
                        ]
                        .len() as u16
                    {
                        self.current_list_index += 1
                    }
                }
                _ => {}
            },
            KeyCode::Up => match self.current_widget {
                TaskWidgets::HISTORY => {
                    if self.current_history_offset > 0 {
                        self.current_history_offset -= 1
                    }
                }
                TaskWidgets::LIST => {
                    if self.current_list_index > 0 {
                        self.current_list_index -= 1
                    }
                }
                _ => {}
            },
            KeyCode::Enter => match self.current_widget {
                TaskWidgets::LIST => {}
                TaskWidgets::HISTORY => {}
                TaskWidgets::CONFIRM => match self.selected_task {
                    Some(OutpostTask::Backup) => {}
                    Some(OutpostTask::Beacon) => {}
                    Some(OutpostTask::PurgeNodes) => {}
                    Some(OutpostTask::PurgeRaw) => {}
                    Some(OutpostTask::PurgePositions) => {}
                    Some(OutpostTask::ReconnectSerial) => {}
                    Some(OutpostTask::Restart) => {}
                    None => {}
                },
            },

            _ => {}
        }
    }

    /// Function to safely update self.status
    fn update_status(&mut self, client_status: ClientStatus) {
        self.status = client_status;
    }
}

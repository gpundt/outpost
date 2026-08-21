use crate::client_http::storage::get_server_tasks;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

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
                    Style::default().fg(Color::White),
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
                Style::default().fg(Color::White),
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

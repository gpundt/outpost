use crate::client_http::storage::get_server_nodes;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub fn generate_nodes_dashbaord_widget<'a>(
    current_scroll_offset: u16,
    selected_widget: bool,
) -> Paragraph<'a> {
    let mut lines = Vec::new();

    let nodes = get_server_nodes();

    match nodes {
        Some(nodes_vec) => {
            if nodes_vec.is_empty() {
                lines.push(Line::from(Span::styled(
                    "No meshtastic nodes discovered...",
                    Style::default().fg(Color::White),
                )));
            }
            for node in nodes_vec {
                let trimmed_long_name = node.node_long_name.trim();
                if trimmed_long_name.is_empty() {
                    continue;
                }
                lines.push(Line::from(Span::styled(
                    format!(
                        "{:<25} : {:<10} ( {:<9} )",
                        trimmed_long_name, node.node_short_name, node.node_id
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
                .title("Nodes")
                .padding(Padding::new(3, 0, 0, 0)),
        )
        .scroll((current_scroll_offset, 0))
}

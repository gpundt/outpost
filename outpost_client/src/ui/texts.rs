use crate::{client_http::storage::get_server_texts, ui::dashboard};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub fn generate_texts_widget<'a>(
    current_scroll_offset: u16,
    selected_widget: bool,
) -> Paragraph<'a> {
    let mut lines = Vec::new();

    let texts = get_server_texts();

    match texts {
        Some(texts_vec) => {
            if texts_vec.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("No meshtastic texts received..."),
                    Style::default().fg(Color::Yellow),
                )));
            }

            for text in texts_vec {
                lines.push(Line::from(Span::styled(
                    format!(
                        "[{}] {:<10}: {}",
                        text.timestamp.format("%m/%d %H:%M:%S"),
                        text.src_id,
                        text.message
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
                .title("Texts")
                .padding(Padding::new(3, 0, 0, 0)),
        )
        .scroll((current_scroll_offset, 0))
}

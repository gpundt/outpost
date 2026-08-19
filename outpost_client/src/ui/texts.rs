use crate::client_http::storage::get_server_texts;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub fn generate_texts_dashboard_widget<'a>() -> Paragraph<'a> {
    let mut lines = Vec::new();

    let texts = get_server_texts();

    match texts {
        Some(texts_vec) => {
            if texts_vec.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("No meshtastic texts received..."),
                    Style::default().fg(Color::White),
                )));
            }
            for text in texts_vec {
                lines.push(Line::from(Span::styled(
                    format!(
                        "[{}] {:<10}: {}\n\n\n",
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
                Style::default().fg(Color::White),
            )));
        }
    }

    Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .title("Texts")
            .padding(Padding::new(5, 0, 0, 0)),
    )
}

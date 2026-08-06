use std::sync::{OnceLock, RwLock};

use chrono::{DateTime, Utc};
use meshtastic::protobufs::{Position, Telemetry, User};

#[derive(Debug, Clone)]
pub enum MessageContent {
    Text(String),
    Telemetry(Telemetry),
    Node(User),
    Position(Position),
    Raw { portnum: String, bytes: Vec<u8> },
}

#[derive(Debug, Clone)]
pub struct ReceivedMessage {
    pub timestamp: DateTime<Utc>,
    pub from: u32,
    pub content: MessageContent,
}

#[derive(Debug, Default)]
pub struct MessageStorage {
    pub texts: Vec<ReceivedMessage>,
    pub telemetry: Vec<ReceivedMessage>,
    pub nodes: Vec<ReceivedMessage>,
    pub other: Vec<ReceivedMessage>,
}

impl MessageStorage {
    pub fn store_message(&mut self, from: u32, message_content: MessageContent) {
        let message = ReceivedMessage {
            timestamp: Utc::now(),
            from,
            content: message_content,
        };

        match &message.content {
            MessageContent::Text(_) => self.texts.push(message),
            MessageContent::Telemetry(_) => self.telemetry.push(message),
            MessageContent::Node(_) | MessageContent::Position(_) => self.nodes.push(message),
            MessageContent::Raw { .. } => self.other.push(message),
        }
    }
}

static MESSAGE_STORAGE: OnceLock<RwLock<MessageStorage>> = OnceLock::new();

pub fn message_storage() -> &'static RwLock<MessageStorage> {
    MESSAGE_STORAGE.get_or_init(|| RwLock::new(MessageStorage::default()))
}

use meshtastic::protobufs::{PortNum, Position, Telemetry, User, from_radio, mesh_packet};
use prost::Message;

use crate::meshtastic::MessageContent;

use super::storage::{ReceivedMessage, message_storage};

/// `packet_receiver` yields `FromRadio` messages. These wrap several different
/// kinds of updates (mesh packets, node info, config, etc.) in `payload_variant`.
pub fn handle_from_radio_packet(from_radio: meshtastic::protobufs::FromRadio) {
    match from_radio.payload_variant {
        Some(from_radio::PayloadVariant::Packet(mesh_packet)) => {
            handle_mesh_packet(mesh_packet);
        }
        Some(from_radio::PayloadVariant::NodeInfo(node_info)) => {
            if let Some(user) = node_info.user {
                message_storage()
                    .write()
                    .unwrap()
                    .store_message(node_info.num, MessageContent::Node(user));
            }
        }
        Some(from_radio::PayloadVariant::MyInfo(_my_info)) => {
            //println!("My node info: {:#?}", my_info);
        }
        Some(_other) => {
            // Config, channel, log records, etc. — print raw if you need to inspect them
            //println!("Other FromRadio variant: {:#?}", other);
        }
        None => {}
    }
}

/// A `MeshPacket` is either `Decoded` (plaintext `Data`, portnum tells you the payload type)
/// or `Encrypted` (raw bytes — only decryptable if you have the channel key, which the
/// radio normally handles for you before it reaches this API).
fn handle_mesh_packet(mesh_packet: meshtastic::protobufs::MeshPacket) {
    let from = mesh_packet.from;
    match mesh_packet.payload_variant {
        Some(mesh_packet::PayloadVariant::Decoded(data)) => {
            let portnum = PortNum::try_from(data.portnum).unwrap_or(PortNum::UnknownApp);

            match portnum {
                PortNum::TextMessageApp => {
                    if let Ok(text) = String::from_utf8(data.payload.clone()) {
                        message_storage()
                            .write()
                            .unwrap()
                            .store_message(from, MessageContent::Text(text));
                    }
                }
                PortNum::PositionApp => {
                    if let Ok(position) = Position::decode(data.payload.as_slice()) {
                        message_storage()
                            .write()
                            .unwrap()
                            .store_message(from, MessageContent::Position(position));
                    }
                }
                PortNum::NodeinfoApp => {
                    if let Ok(user) = User::decode(data.payload.as_slice()) {
                        message_storage()
                            .write()
                            .unwrap()
                            .store_message(from, MessageContent::Node(user));
                    }
                }
                PortNum::TelemetryApp => {
                    if let Ok(telemetry) = Telemetry::decode(data.payload.as_slice()) {
                        message_storage()
                            .write()
                            .unwrap()
                            .store_message(from, MessageContent::Telemetry(telemetry));
                    }
                }
                other => {
                    message_storage().write().unwrap().store_message(
                        from,
                        MessageContent::Raw {
                            portnum: format!("{:?}", other),
                            bytes: data.payload,
                        },
                    );
                }
            }
        }
        Some(mesh_packet::PayloadVariant::Encrypted(_)) => {
            message_storage().write().unwrap().store_message(
                from,
                MessageContent::Raw {
                    portnum: "Encrypted".to_string(),
                    bytes: Vec::new(),
                },
            );
        }
        None => {}
    }
}

pub fn format_message(message: &ReceivedMessage) -> String {
    let time = message.timestamp.format("%H:%M:%S");
    let from = format!("{:#010x}", message.from);

    match &message.content {
        MessageContent::Text(text) => format!("[{time}] {from}: {text}"),
        MessageContent::Telemetry(t) => format!("[{time}] {from}: {:?}", t),
        MessageContent::Node(user) => {
            format!("[{time}] {from}: {} ({})", user.long_name, user.short_name,)
        }
        MessageContent::Position(pos) => {
            let lat = pos.latitude_i.map(|x| x as f64).unwrap() / 1e7;
            let lon = pos.longitude_i.map(|x| x as f64).unwrap() / 1e7;
            format!("[{time}] {from}: lat={lat:.5}, lon={lon:.5}")
        }
        MessageContent::Raw { portnum, bytes } => {
            format!("[{time}] {from}: {portnum} ({} bytes)", bytes.len())
        }
    }
}

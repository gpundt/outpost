use log::error;
use meshtastic::protobufs::{NodeInfo, PortNum, Position, from_radio, mesh_packet};
use prost::Message;

use crate::database::{
    insert_meshtastic_node, insert_meshtastic_position, insert_meshtastic_raw,
    insert_meshtastic_telemetry, insert_meshtastic_text,
};

/// `packet_receiver` yields `FromRadio` messages. These wrap several different
/// kinds of updates (mesh packets, node info, config, etc.) in `payload_variant`.
pub async fn handle_from_radio_packet(from_radio: meshtastic::protobufs::FromRadio) {
    match from_radio.payload_variant {
        Some(from_radio::PayloadVariant::Packet(mesh_packet)) => {
            handle_mesh_packet(mesh_packet).await;
        }
        Some(from_radio::PayloadVariant::NodeInfo(node_info)) => {
            let _ = insert_meshtastic_node(node_info).await;
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
async fn handle_mesh_packet(mesh_packet: meshtastic::protobufs::MeshPacket) {
    match mesh_packet.clone().payload_variant {
        Some(mesh_packet::PayloadVariant::Decoded(data)) => {
            let portnum = PortNum::try_from(data.portnum).unwrap_or(PortNum::UnknownApp);

            match portnum {
                PortNum::TextMessageApp => match String::from_utf8(data.payload.clone()) {
                    Ok(text) => {
                        error!("TEXT RECEIVED: {}", text);
                        let _ = insert_meshtastic_text(
                            format!("{}", data.source),
                            format!("{}", data.dest),
                            &text,
                        )
                        .await;
                    }
                    Err(e) => {
                        error!("Failed to decode PortNum::TextMessageApp: {}", e);
                    }
                },
                PortNum::PositionApp => match Position::decode(data.payload.as_slice()) {
                    Ok(p) => {
                        error!("POSITON RECEIVED: {:?}", p);
                        let _ = insert_meshtastic_position(p).await;
                    }
                    Err(e) => error!("Failed to decode Portnum::PositionApp: {}", e),
                },
                PortNum::NodeinfoApp => match NodeInfo::decode(data.payload.as_slice()) {
                    Ok(n) => {
                        error!("NODE RECEIVED: {:?}", n);
                        let _ = insert_meshtastic_node(n).await;
                    }
                    Err(e) => error!("Failed to decode PortNum::NodeinfoApp: {}", e),
                },
                PortNum::TelemetryApp => {
                    error!("TELEMETRY RECEIVED");
                    let _ = insert_meshtastic_telemetry().await;
                }
                _ => {
                    let _ = insert_meshtastic_raw(mesh_packet, false);
                }
            }
        }
        Some(mesh_packet::PayloadVariant::Encrypted(_)) => {
            error!("ENCRYPTED RECEIVED");
            let _ = insert_meshtastic_raw(mesh_packet, true).await;
        }
        None => {}
    }
}

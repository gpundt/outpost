use chrono::Utc;
use log::error;
use meshtastic::protobufs::{
    DeviceMetrics, NodeInfo, PortNum, Position, User, from_radio, mesh_packet,
};
use prost::Message;

use crate::database::insert::{
    insert_meshtastic_node, insert_meshtastic_position, insert_meshtastic_raw,
    insert_meshtastic_telemetry, insert_meshtastic_text,
};
use crate::database::schema::{
    MeshtasticNodeEntry, MeshtasticPositionEntry, MeshtasticRawEntry, MeshtasticTextEntry,
};

/// `packet_receiver` yields `FromRadio` messages. These wrap several different
/// kinds of updates (mesh packets, node info, config, etc.) in `payload_variant`.
pub async fn handle_from_radio_packet(from_radio: meshtastic::protobufs::FromRadio) {
    match from_radio.payload_variant {
        Some(from_radio::PayloadVariant::Packet(mesh_packet)) => {
            handle_mesh_packet(mesh_packet).await;
        }
        Some(from_radio::PayloadVariant::NodeInfo(node_info)) => {
            let _ = insert_meshtastic_node(MeshtasticNodeEntry {
                id: 0,
                node_num: node_info.num,
                node_id: node_info.clone().user.unwrap_or(User::default()).id,
                long_name: node_info.clone().user.unwrap_or(User::default()).long_name,
                short_name: node_info.clone().user.unwrap_or(User::default()).short_name,
                hw_model: node_info.clone().user.unwrap_or(User::default()).hw_model,
                role: node_info.clone().user.unwrap_or(User::default()).role,
                last_heard: node_info.last_heard,
                uptime: node_info
                    .device_metrics
                    .unwrap_or(DeviceMetrics::default())
                    .uptime_seconds
                    .unwrap_or(0),
                channel: node_info.channel,
                hops_away: node_info.hops_away.unwrap_or(0),
            })
            .await;
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
                        let _ = insert_meshtastic_text(&MeshtasticTextEntry {
                            id: 0,
                            timestamp: Utc::now().naive_utc(),
                            src_id: mesh_packet.from,
                            dst_id: mesh_packet.to,
                            message: text,
                        })
                        .await;
                    }
                    Err(e) => {
                        error!("Failed to decode PortNum::TextMessageApp: {}", e);
                    }
                },
                PortNum::PositionApp => match Position::decode(data.payload.as_slice()) {
                    Ok(position_info) => {
                        error!("POSITON RECEIVED: {:?}", position_info);
                        let _ = insert_meshtastic_position(MeshtasticPositionEntry {
                            id: 0,
                            latitude: position_info.latitude_i(),
                            longitude: position_info.longitude_i(),
                            altitude: position_info.altitude(),
                            time: position_info.time,
                            timestamp: position_info.timestamp,
                            next_update: position_info.next_update,
                        })
                        .await;
                    }
                    Err(e) => error!("Failed to decode Portnum::PositionApp: {}", e),
                },
                PortNum::NodeinfoApp => match NodeInfo::decode(data.payload.as_slice()) {
                    Ok(node_info) => {
                        error!("NODE RECEIVED: {:?}", node_info);
                        let _ = insert_meshtastic_node(MeshtasticNodeEntry {
                            id: 0,
                            node_num: node_info.num,
                            node_id: node_info.clone().user.unwrap_or(User::default()).id,
                            long_name: node_info.clone().user.unwrap_or(User::default()).long_name,
                            short_name: node_info
                                .clone()
                                .user
                                .unwrap_or(User::default())
                                .short_name,
                            hw_model: node_info.clone().user.unwrap_or(User::default()).hw_model,
                            role: node_info.clone().user.unwrap_or(User::default()).role,
                            last_heard: node_info.last_heard,
                            uptime: node_info
                                .device_metrics
                                .unwrap_or(DeviceMetrics::default())
                                .uptime_seconds
                                .unwrap_or(0),
                            channel: node_info.channel,
                            hops_away: node_info.hops_away.unwrap_or(0),
                        })
                        .await;
                    }
                    Err(e) => error!("Failed to decode PortNum::NodeinfoApp: {}", e),
                },
                PortNum::TelemetryApp => {
                    let _ = insert_meshtastic_telemetry().await;
                }
                _ => {
                    let _ = insert_meshtastic_raw(MeshtasticRawEntry {
                        id: 0,
                        src_node: mesh_packet.from,
                        dst_node: mesh_packet.to,
                        channel: mesh_packet.channel,
                        hop_limit: mesh_packet.hop_limit,
                        hop_start: mesh_packet.hop_start,
                        next_hop: mesh_packet.hop_start,
                        encrypted: false,
                    });
                }
            }
        }
        Some(mesh_packet::PayloadVariant::Encrypted(_)) => {
            let _ = insert_meshtastic_raw(MeshtasticRawEntry {
                id: 0,
                src_node: mesh_packet.from,
                dst_node: mesh_packet.to,
                channel: mesh_packet.channel,
                hop_limit: mesh_packet.hop_limit,
                hop_start: mesh_packet.hop_start,
                next_hop: mesh_packet.hop_start,
                encrypted: true,
            })
            .await;
        }
        None => {}
    }
}

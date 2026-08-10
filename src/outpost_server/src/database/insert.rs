use super::schema::get_db_pool;
use chrono::Utc;
use log::{debug, error};

pub async fn insert_meshtastic_text(
    src_id: String,
    dst_id: String,
    message: &str,
) -> Result<(), sqlx::Error> {
    let timestamp: chrono::DateTime<Utc> = Utc::now();
    match sqlx::query(
        r#"
        INSERT INTO meshtastic_texts (timestamp, src_id, dst_id, message)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(timestamp)
    .bind(src_id)
    .bind(dst_id)
    .bind(message)
    .execute(get_db_pool())
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            error!("Failed to insert into meshtastic_texts: {}", e);
            return Err(e);
        }
    }
}

pub async fn insert_meshtastic_node(
    node_info: meshtastic::protobufs::NodeInfo,
) -> Result<(), sqlx::Error> {
    match sqlx::query(
        r#"
        INSERT INTO meshtastic_nodes (node_num, node_id, node_long_name, node_short_name, hw_model, role, is_unmessagable, latitude, longitude, last_heard, uptime, channel, hops_away)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
        "#
    )
        .bind(node_info.num)
        .bind(node_info.clone().user.unwrap().id)
        .bind(node_info.clone().user.unwrap().long_name)
        .bind(node_info.clone().user.unwrap().short_name)
        .bind(node_info.clone().user.unwrap().hw_model)
        .bind(node_info.clone().user.unwrap().role)
        .bind(node_info.clone().user.unwrap().is_unmessagable)
        .bind(node_info.position.unwrap().latitude_i)
        .bind(node_info.position.unwrap().longitude_i)
        .bind(node_info.last_heard)
        .bind(node_info.device_metrics.unwrap().uptime_seconds)
        .bind(node_info.channel)
        .bind(node_info.hops_away)
        .execute(get_db_pool())
        .await {
        Ok(_) => return Ok(()),
        Err(e) => {
            error!("Failed to insert into 'meshtastic_nodes' table: {}", e);
            return Err(e);
        }
    };
}

pub async fn insert_meshtastic_position(
    position: meshtastic::protobufs::Position,
) -> Result<(), sqlx::Error> {
    match sqlx::query(
        r#"
        INSERT INTO meshtastic_positions (latitude, longitude, altitude, time, timestamp, next_update)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(position.latitude_i.unwrap())
    .bind(position.longitude_i.unwrap())
    .bind(position.altitude.unwrap())
    .bind(position.time)
    .bind(position.timestamp)
    .bind(position.next_update)
    .execute(get_db_pool())
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            error!("Failed to insert into 'meshastic_position' table: {}", e);
            return Err(e);
        }
    };
}

pub async fn insert_meshtastic_telemetry() -> Result<(), sqlx::Error> {
    debug!("Telemetry packet intercepted... Not adding to database");
    Ok(())
}

pub async fn insert_meshtastic_raw(
    mesh_packet: meshtastic::protobufs::MeshPacket,
    encrypted: bool,
) -> Result<(), sqlx::Error> {
    match sqlx::query(
        r#"
        INSERT INTO meshtastic_raw (src_node, dst_node, channel, hop_limit, hop_start, next_hop, encrypted)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(mesh_packet.from)
    .bind(mesh_packet.to)
    .bind(mesh_packet.channel)
    .bind(mesh_packet.hop_limit)
    .bind(mesh_packet.hop_start)
    .bind(mesh_packet.next_hop)
    .bind(encrypted)
    .execute(get_db_pool())
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            error!("Failed to insert into 'meshtastic_raw' table: {}", e);
            return Err(e);
        }
    }
}

pub async fn insert_http_request(
    method: &str,
    source: &str,
    endpoint: &str,
    user_agent: &str,
    status_code: u16,
) -> Result<(), sqlx::Error> {
    match sqlx::query(
        r#"
        INSERT INTO http_requests (method, source, endpoint, user_agent, status_code)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(method)
    .bind(source)
    .bind(endpoint)
    .bind(user_agent)
    .bind(status_code)
    .execute(get_db_pool())
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            error!("Failed to insert into 'http_requests' table: {}", e);
            return Err(e);
        }
    };
}

pub async fn insert_task_request_start(task_type: &str) -> Result<i64, sqlx::Error> {
    let requested_at: chrono::DateTime<Utc> = Utc::now();

    let result = match sqlx::query(
        r#"
        INSERT INTO tasks (type, requested_at)
        VALUES (?, ?)
        "#,
    )
    .bind(task_type)
    .bind(requested_at)
    .execute(get_db_pool())
    .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to insert into 'tasks' table: {}", e);
            return Err(e);
        }
    };

    let generated_id = result.last_insert_rowid();

    Ok(generated_id)
}

pub async fn insert_task_request_finish(row_id: i64, successful: bool) -> Result<(), sqlx::Error> {
    let finished_at: chrono::DateTime<Utc> = Utc::now();

    match sqlx::query(
        r#"
        UPDATE tasks SET finished_at = ?, successful = ?
        WHERE id = ?
        "#,
    )
    .bind(finished_at)
    .bind(successful)
    .bind(row_id)
    .execute(get_db_pool())
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            error!("Failed to update 'tasks' table with finished task: {}", e);
            return Err(e);
        }
    };
}

use super::schema::get_db_pool;
use super::schema::{
    HTTPRequestEntry, MeshtasticNodeEntry, MeshtasticPositionEntry, MeshtasticRawEntry,
    MeshtasticTextEntry,
};
use chrono::Utc;
use log::{debug, error, trace};

pub async fn insert_meshtastic_text(text_entry: &MeshtasticTextEntry) -> Result<(), sqlx::Error> {
    match sqlx::query(
        r#"
        INSERT INTO meshtastic_texts (timestamp, src_id, dst_id, message)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(text_entry.timestamp)
    .bind(text_entry.src_id)
    .bind(text_entry.dst_id)
    .bind(text_entry.message.as_str())
    .execute(get_db_pool())
    .await
    {
        Ok(_) => {
            trace!("INSERT INTO meshastic_texts VALUES {:?}", text_entry);
            return Ok(());
        }
        Err(e) => {
            error!("Failed to insert into meshtastic_texts: {}", e);
            return Err(e);
        }
    }
}

pub async fn insert_meshtastic_node(node_entry: MeshtasticNodeEntry) -> Result<(), sqlx::Error> {
    let entry_clone = node_entry.clone();
    match sqlx::query(
        r#"
        INSERT INTO meshtastic_nodes (node_num, node_id, node_long_name, node_short_name, hw_model, role, last_heard, uptime, channel, hops_away)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
        "#
    )
        .bind(node_entry.node_num)
        .bind(node_entry.node_id)
        .bind(node_entry.long_name)
        .bind(node_entry.short_name)
        .bind(node_entry.hw_model)
        .bind(node_entry.role)
        .bind(node_entry.last_heard)
        .bind(node_entry.uptime)
        .bind(node_entry.channel)
        .bind(node_entry.hops_away)
        .execute(get_db_pool())
        .await {
        Ok(_) => {
            trace!("INSERT INTO meshtastic_nodes VALUES {:?}", entry_clone);
            return Ok(());
        }
            ,
        Err(e) => {
            error!("Failed to insert into 'meshtastic_nodes' table: {}", e);
            return Err(e);
        }
    };
}

pub async fn insert_meshtastic_position(
    position_entry: MeshtasticPositionEntry,
) -> Result<(), sqlx::Error> {
    match sqlx::query(
        r#"
        INSERT INTO meshtastic_positions (latitude, longitude, altitude, time, timestamp, next_update)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(position_entry.latitude)
    .bind(position_entry.longitude)
    .bind(position_entry.altitude)
    .bind(position_entry.time)
    .bind(position_entry.timestamp)
    .bind(position_entry.next_update)
    .execute(get_db_pool())
    .await
    {
        Ok(_) => {
            trace!("INSERT INTO meshtastic_positions VALUES {:?}", position_entry);
            return Ok(());
        },
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

pub async fn insert_meshtastic_raw(raw_entry: MeshtasticRawEntry) -> Result<(), sqlx::Error> {
    match sqlx::query(
        r#"
        INSERT INTO meshtastic_raw (src_node, dst_node, channel, hop_limit, hop_start, next_hop, encrypted)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(raw_entry.src_node)
    .bind(raw_entry.dst_node)
    .bind(raw_entry.channel)
    .bind(raw_entry.hop_limit)
    .bind(raw_entry.hop_start)
    .bind(raw_entry.next_hop)
    .bind(raw_entry.encrypted)
    .execute(get_db_pool())
    .await
    {
        Ok(_) => {
            trace!("INSERT INTO meshtastic_raw VALUES {:?}", raw_entry);
            return Ok(());
        },
        Err(e) => {
            error!("Failed to insert into 'meshtastic_raw' table: {}", e);
            return Err(e);
        }
    }
}

pub async fn insert_http_request(request_entry: HTTPRequestEntry) -> Result<(), sqlx::Error> {
    let entry_clone = request_entry.clone();
    match sqlx::query(
        r#"
        INSERT INTO http_requests (method, source, endpoint, user_agent, status_code, timestamp)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(request_entry.method)
    .bind(request_entry.source)
    .bind(request_entry.endpoint)
    .bind(request_entry.user_agent)
    .bind(request_entry.status_code)
    .bind(request_entry.timestamp)
    .execute(get_db_pool())
    .await
    {
        Ok(_) => {
            trace!("INSERT INTO http_requests VALUES {:?}", entry_clone);
            return Ok(());
        }
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

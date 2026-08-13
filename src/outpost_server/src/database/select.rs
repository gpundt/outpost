use crate::database::schema::{
    HTTPRequestEntry, MeshtasticNodeEntry, MeshtasticPositionEntry, MeshtasticRawEntry,
    MeshtasticTextEntry, get_db_pool,
};

use log::{error, trace};

/// Function to select X rows from the http_requests db table
pub async fn select_http_requests_by_count(
    count: u32,
) -> Result<Vec<HTTPRequestEntry>, sqlx::Error> {
    match sqlx::query_as::<_, HTTPRequestEntry>(
        "SELECT id, method, source, endpoint, user_agent, status_code, timestamp FROM http_requests ORDER BY id DESC LIMIT ?",
    ).bind(count).fetch_all(get_db_pool()).await {
        Ok(requests) => {
            trace!("SELECT FROM http_requests ORDER BY id DESC LIMIT {}", count);
            return Ok(requests);
        },
        Err(e) => {
            error!("Failed to query {} texts from 'http_requests' table: {}", count, e);
            return Err(e);
        }
    };
}

/// Function to select X rows from the meshtastic_texts db table
pub async fn select_meshtastic_texts_by_count(
    count: u32,
) -> Result<Vec<MeshtasticTextEntry>, sqlx::Error> {
    match sqlx::query_as::<_, MeshtasticTextEntry>(
        "SELECT id, timestamp, src_id, dst_id, message FROM meshtastic_texts ORDER BY id DESC LIMIT ?",
    ).bind(count).fetch_all(get_db_pool()).await {
        Ok(texts) => {
            trace!("SELECT FROM meshtastic_texts ORDER BY id DESC LIMIT {}", count);
            return Ok(texts);
        },
        Err(e) => {
            error!("Failed to query {} texts from 'meshtastic_texts' table: {}", count, e);
            return Err(e);
        }
    };
}

/// Function to select all rows from the meshtastic_nodes db table
pub async fn select_meshtastic_nodes() -> Result<Vec<MeshtasticNodeEntry>, sqlx::Error> {
    match sqlx::query_as::<_, MeshtasticNodeEntry>(
        "SELECT id, node_num, node_id, node_long_name, node_short_name, hw_model, role, last_heard, uptime, channel, hops_away FROM meshtastic_nodes",
    ).fetch_all(get_db_pool()).await {
        Ok(nodes) => {
            trace!("SELECT FROM meshtastic_nodes");
            return Ok(nodes);
        },
        Err(e) => {
            error!("Failed to query nodes from 'meshtastic_nodes' table: {}", e);
            return Err(e);
        }
    };
}

/// Function to select all rows from the meshtastic_positions db table
pub async fn select_meshtastic_positions() -> Result<Vec<MeshtasticPositionEntry>, sqlx::Error> {
    match sqlx::query_as::<_, MeshtasticPositionEntry>(
        "SELECT id, latitude, longitude, altitude, time, timestamp, next_update FROM meshtastic_positions",
    ).fetch_all(get_db_pool()).await {
        Ok(positions) => {
            trace!("SELECT FROM meshtastic_positions");
            return Ok(positions);
        }
        Err(e) => {
            error!("Failed to query positions from 'meshtastic_positions' table: {}", e);
            return Err(e);
        }
    }
}

/// Function to select X rows from the meshtastic_raw db table
pub async fn select_meshtastic_raw_by_count(
    count: u32,
) -> Result<Vec<MeshtasticRawEntry>, sqlx::Error> {
    match sqlx::query_as::<_, MeshtasticRawEntry>(
        "SELECT id, src_node, dst_node, channel, hop_limit, hop_start, next_hop, encrypted FROM meshtastic_raw ORDER BY id DESC LIMIT ?",
    ).bind(count).fetch_all(get_db_pool()).await {
        Ok(raw) => {
            trace!("SELECT FROM meshtastic_raw ORDER BY id DESC LIMIT {}", count);
            return Ok(raw);
        }
        Err(e) => {
            error!("Failed to query {} raw packets from 'meshtastic_raw' table: {}", count, e);
            return Err(e);
        }
    }
}

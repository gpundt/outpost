use chrono::{NaiveDateTime, Utc};
use config::files::DATABASE_DIR;
use log::{debug, error, info};
use sqlx::{
    FromRow, Pool, QueryBuilder, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use std::{str::FromStr, sync::OnceLock};
// Static container for the global
static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn initialize_database() -> Result<String, sqlx::Error> {
    let database_url = format!("sqlite://{}outpost_server.db", DATABASE_DIR);

    debug!("URL: {}", database_url);
    let options = SqliteConnectOptions::from_str(database_url.as_str())?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    DB_POOL
        .set(pool)
        .map_err(|_| sqlx::Error::Configuration("Pool already set".into()))?;

    // ── http_requests Table ─────
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS http_requests (
            id          INTEGER  PRIMARY KEY AUTOINCREMENT,
            method      TEXT     NOT NULL,
            source      TEXT     NOT NULL,
            endpoint    TEXT     NOT NULL,
            user_agent  TEXT     NOT NULL,
            status_code INTEGER  NOT NULL,
            timestamp   DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(get_db_pool())
    .await?;

    // ── tasks Table ────────────
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id            INTEGER  PRIMARY KEY AUTOINCREMENT,
            type          TEXT     NOT NULL,
            requested_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            finished_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
            successful    BOOLEAN  NOT NULL
        )
        "#,
    )
    .execute(get_db_pool())
    .await?;

    // ── meshtastic_texts Table ──────
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS meshtastic_texts (
            id         INTEGER  PRIMARY KEY AUTOINCREMENT,
            timestamp  DATETIME DEFAULT CURRENT_TIMESTAMP,
            src_id     INTEGER  NOT NULL,
            dst_id     INTEGER  NOT NULL,
            message    TEXT     NOT NULL
        )
        "#,
    )
    .execute(get_db_pool())
    .await?;

    // ── meshtastic_position Table ───
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS meshtastic_positions (
            id             INTEGER PRIMARY KEY AUTOINCREMENT,
            latitude       INTEGER NOT NULL,
            longitude      INTEGER NOT NULL,
            altitude       INTEGER NOT NULL,
            time           INTEGER NOT NULL,
            timestamp      INTEGER NOT NULL,
            next_update    INTEGER NOT NULL
        )
        "#,
    )
    .execute(get_db_pool())
    .await?;

    // ── meshtastic_nodes Table ──────
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS meshtastic_nodes (
            id              INTEGER  PRIMARY KEY AUTOINCREMENT,
            node_num        INTEGER  NOT NULL,
            node_id         TEXT     NOT NULL,
            node_long_name  TEXT     NOT NULL,
            node_short_name TEXT     NOT NULL,
            hw_model        INTEGER  NOT NULL,
            role            INTEGER  NOT NULL,
            last_heard      INTEGER  NOT NULL,
            uptime          INTEGER  NOT NULL,
            channel         INTEGER  NOT NULL,
            hops_away       INTEGER  NOT NULL
        )
        "#,
    )
    .execute(get_db_pool())
    .await?;

    // ── meshtastic_telemetry Table ──
    // NOT SUPPORTED AT THIS TIME

    // ── meshtastic_raw Table ──────
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS mestastic_raw (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            src_node  INTEGER NOT NULL,
            dst_node  INTEGER NOT NULL,
            channel   INTEGER NOT NULL,
            hop_limit INTEGER NOT NULL,
            hop_start INTEGER NOT NULL,
            next_hop  INTEGER NOT NULL,
            encrypted BOOLEAN NOT NULL
        )
        "#,
    )
    .execute(get_db_pool())
    .await?;

    Ok(database_url)
}

#[derive(Debug, Default, FromRow)]
pub struct HTTPRequestEntry {
    pub id: i32,
    pub method: String,
    pub source: String,
    pub endpoint: String,
    pub user_agent: String,
    pub status_code: u16,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Default, FromRow)]
pub struct TaskRequestEntry {
    pub id: i32,
    pub task_type: String,
    pub requested_at: NaiveDateTime,
    pub finished_at: NaiveDateTime,
    pub successful: bool,
}
#[derive(Debug, Default, FromRow)]
pub struct MeshtasticTextEntry {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub src_id: u32,
    pub dst_id: u32,
    pub message: String,
}
#[derive(Debug, Default, FromRow)]
pub struct MeshtasticPositionEntry {
    pub id: i32,
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub time: u32,
    pub timestamp: u32,
    pub next_update: u32,
}
#[derive(Debug, Default, FromRow)]
pub struct MeshtasticNodeEntry {
    pub id: i32,
    pub node_num: u32,
    pub node_id: String,
    pub long_name: String,
    pub short_name: String,
    pub hw_model: i32,
    pub role: i32,
    pub last_heard: u32,
    pub uptime: u32,
    pub channel: u32,
    pub hops_away: u32,
}
#[derive(Debug, Default, FromRow)]
pub struct MeshtasticRawEntry {
    pub id: i32,
    pub src_node: u32,
    pub dst_node: u32,
    pub channel: u32,
    pub hop_limit: u32,
    pub hop_start: u32,
    pub next_hop: u32,
    pub encrypted: bool,
}

pub fn get_db_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database pool is not initialized")
}

pub async fn is_db_connected() -> bool {
    let pool: &Pool<Sqlite> = get_db_pool();

    if pool.is_closed() {
        return false;
    }

    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn backup_database() -> Result<String, sqlx::Error> {
    let path_str = format!(
        "{}outpost_server_{}.db",
        DATABASE_DIR,
        Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string()
    );

    let mut vacuum_query: QueryBuilder<Sqlite> = QueryBuilder::new("VACUUM INTO ");
    let escaped_path = path_str.replace('\'', "''");
    vacuum_query.push(format!("'{}'", escaped_path));
    let query = vacuum_query.build();

    match query.execute(get_db_pool()).await {
        Ok(_) => {
            info!("Database backup created: {}", path_str);
            return Ok(path_str.to_string());
        }
        Err(e) => {
            error!("Failed to create database backup: {}", e);
            return Err(e);
        }
    };
}

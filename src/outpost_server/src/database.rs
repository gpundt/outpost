use chrono::Utc;
use config::files::DATABASE_DIR;
use log::{debug, error, info};
use sqlx::{
    Pool, QueryBuilder, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use std::{str::FromStr, sync::OnceLock};
// Static container for the global
static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init_database() -> Result<String, sqlx::Error> {
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
            status_code INTEGER  NOT NULL DEFAULT 0,
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
            type          TEXT     NOT_NULL,
            requested_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            finished_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
            successful    INTEGER  NOT NULL DEFAULT 0 CHECK (successful IN (0, 1))
        )
        "#,
    )
    .execute(get_db_pool())
    .await?;

    Ok(database_url)
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
            error!("Failed to insert into http_requests table: {}", e);
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
            error!("Failed to insert into tasks table: {}", e);
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
            error!("Failed to update tasks table with finished task: {}", e);
            return Err(e);
        }
    };
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

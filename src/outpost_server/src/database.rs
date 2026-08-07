use config::files::DATABASE_DIR;
use log::debug;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
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

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS http_requests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            method TEXT NOT NULL,
            source TEXT NOT NULL,
            endpoint TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
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

use chrono::Utc;
use config::files::DATABASE_DIR;
use log::{debug, info};
use sqlx::{
    QueryBuilder, Sqlite,
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
    Ok(database_url)
}

pub fn get_db_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database pool is not initialized")
}

pub async fn insert_http_request(
    method: &str,
    source: &str,
    endpoint: &str,
    user_agent: &str,
    status_code: u16,
) -> Result<(), sqlx::Error> {
    sqlx::query(
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
    .await?;

    Ok(())
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

    query.execute(get_db_pool()).await?;

    info!("Database backup created: {}", path_str);
    Ok(path_str.to_string())
}

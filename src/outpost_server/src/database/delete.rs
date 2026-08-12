use log::{error, trace, warn};
use sqlx::{QueryBuilder, Sqlite};

use super::schema::get_db_pool;

pub async fn delete_from_table(table_name: &str) -> Result<(), sqlx::Error> {
    let mut delete_query: QueryBuilder<Sqlite> = QueryBuilder::new("DELETE FROM ");
    delete_query.push(format!("{}", table_name));
    let query = delete_query.build();

    match query.execute(get_db_pool()).await {
        Ok(_) => {
            trace!("DELETE FROM {}", table_name);
            warn!("Deleted all entries in '{}' table", table_name);
            Ok(())
        }
        Err(e) => {
            error!(
                "Failed to delete all entries in '{}' table: {}",
                table_name, e
            );
            return Err(e);
        }
    }
}

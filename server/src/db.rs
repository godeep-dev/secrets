//! Database

use std::path::Path;

use anyhow::anyhow;
use sqlx::{sqlite::SqlitePoolOptions, Executor, Pool, Sqlite};

pub mod orgs;
pub mod projects;
pub mod secrets;
pub mod users;

/// DB connection pool
pub type DbConn = Pool<Sqlite>;

/// Returns a database connection pool
pub async fn conn_pool(db_path: &Path) -> anyhow::Result<DbConn> {
    let db_path_str = db_path.to_str().ok_or_else(|| anyhow!("Invalid DB path"))?;
    let db_conn_str = format!("sqlite:{db_path_str}");
    Ok(SqlitePoolOptions::new().connect(&db_conn_str).await?)
}

/// Initializes the DB
///
/// This creates the schema
pub async fn init(db: &DbConn) -> anyhow::Result<()> {
    db.execute("PRAGMA foreign_keys = ON;").await?;
    users::create_table(db).await?;
    orgs::create_table(db).await?;
    projects::create_table(db).await?;
    secrets::create_table(db).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use tokio::sync::OnceCell;

    /// Returns the dn connection pool
    async fn db_handle() -> anyhow::Result<DbConn> {
        /// DB connection
        static DB: OnceCell<DbConn> = OnceCell::const_new();

        let config = Config::default();
        DB.get_or_try_init(|| async { conn_pool(&config.database).await })
            .await
            .cloned()
    }

    #[tokio::test]
    async fn init_db() -> anyhow::Result<()> {
        let db = db_handle().await?;
        init(&db).await
    }
}

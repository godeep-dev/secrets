//! DB users

use super::DbConn;

/// Create the `users` table
pub(super) async fn create_table(db: &DbConn) -> anyhow::Result<()> {
    let _res = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        );",
    )
    .execute(db)
    .await?;

    Ok(())
}

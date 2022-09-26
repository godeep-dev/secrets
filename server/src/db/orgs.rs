//! DB organizations

use super::DbConn;

/// Create the `organizations` table
pub(super) async fn create_table(db: &DbConn) -> anyhow::Result<()> {
    let _res = sqlx::query(
        "CREATE TABLE IF NOT EXISTS organizations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );",
    )
    .execute(db)
    .await?;

    Ok(())
}

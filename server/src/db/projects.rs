//! DB projects

use super::DbConn;

/// Create the `projects` table
pub(super) async fn create_table(db: &DbConn) -> anyhow::Result<()> {
    let _res = sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );",
    )
    .execute(db)
    .await?;

    Ok(())
}

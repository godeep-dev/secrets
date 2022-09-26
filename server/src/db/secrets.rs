//! DB secrets

use super::DbConn;

/// Create the `secrets` table
pub(super) async fn create_table(db: &DbConn) -> anyhow::Result<()> {
    let _res = sqlx::query(
        "CREATE TABLE IF NOT EXISTS secrets (
            id INTEGER PRIMARY KEY,
            key TEXT NOT NULL,
            value TEXT NOT NULL,
            organization_id INTEGER NOT NULL,
            project_id INTEGER,
            FOREIGN KEY (organization_id) REFERENCES organizations (id),
            FOREIGN KEY (project_id) REFERENCES projects (id)
        );",
    )
    .execute(db)
    .await?;

    Ok(())
}

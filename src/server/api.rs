//! Server API

use rusqlite::params;

use crate::shared::{SignupPayload, User};

use super::DbPool;

/// Initializes the database
pub fn init_db(db_pool: &DbPool) -> anyhow::Result<()> {
    let client = db_pool.get()?;

    // Enable foregn key support
    client.execute("PRAGMA foreign_keys = ON", params![])?;

    // Users table
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL
    )",
        params![],
    )?;

    // Table projects
    client.execute(
        "CREATE TABLE IF NOT EXISTS projects (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )",
        params![],
    )?;

    // Table organizations
    client.execute(
        "CREATE TABLE IF NOT EXISTS organizations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        params![],
    )?;

    // Table secrets
    client.execute(
        "CREATE TABLE IF NOT EXISTS secrets (
        id INTEGER PRIMARY KEY,
        organization_id TEXT NOT NULL,
        project_id TEXT,
        key TEXT NOT NULL,
        value TEXT NOT NULL,
        FOREIGN KEY (organization_id) REFERENCES organizations (id),
        FOREIGN KEY (project_id) REFERENCES projects (id)
    )",
        params![],
    )?;

    Ok(())
}

/// Signups a new user
pub fn auth_signup(payload: SignupPayload) -> anyhow::Result<()> {
    // Check if the user exists
    Ok(())
}

// /// Adds a secret
// pub fn add_secret(state: &ServerState, key: &str, value: &str) -> anyhow::Result<()> {
//     let db_client = state.db_pool.get()?;
//     let db_pool = db_client.execute("", params![])?;
//     Ok(())
// }

// /// List the secrets
// pub fn list_secrets(state: &ServerState, key: &str, value: &str) -> anyhow::Result<()> {
//     ServerStatus { port: state.port }
// }

// /// Removes a secret
// pub fn rm_secret(state: &ServerState, key: &str, value: &str) -> anyhow::Result<()> {
//     ServerStatus { port: state.port }
// }

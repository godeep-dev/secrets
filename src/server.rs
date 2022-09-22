//! Server

use std::{fs, net::SocketAddr, path::PathBuf};

use anyhow::anyhow;
use axum::{routing::get, Extension};
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};

mod api;
mod config;
mod routes;

pub use config::*;

/// Server
#[derive(Debug)]
pub struct Server {
    /// Port
    pub port: u16,
    /// Path to the database file (`****.db`)
    pub database: PathBuf,
}

/// Server info
#[derive(Debug, Clone, Serialize)]
pub struct ServerInfo {
    /// Port
    pub port: u16,
    /// Path to the database
    pub database: PathBuf,
}

/// DB client
pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

impl Server {
    /// Instantiates the [Server]
    pub fn new(config: ServerConfig) -> Self {
        Server {
            port: config.port,
            database: config.database,
        }
    }

    /// Returns the server address
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.port))
    }

    /// Initializes the server
    pub async fn init(&self) -> anyhow::Result<()> {
        if !self.database.exists() {
            fs::write(&self.database, "")?;
        }

        let db_pool = self.db_pool()?;
        api::init_db(&db_pool)?;

        Ok(())
    }

    /// Starts the server
    pub async fn start(self) -> anyhow::Result<()> {
        // Set the server info
        let info = ServerInfo {
            port: self.port,
            database: self.database.clone(),
        };

        // Set the server db pool
        let db_pool = self.db_pool()?;

        // Configure the router
        let app = axum::Router::new()
            .route("/status", get(routes::get_server_status))
            .layer(Extension(info))
            .layer(Extension(db_pool));

        // Start the server
        let addr = self.addr();
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    /// Returns a db pool
    fn db_pool(&self) -> anyhow::Result<DbPool> {
        let db_path = &self.database;
        let db_manager = SqliteConnectionManager::file(db_path);
        Ok(r2d2::Pool::new(db_manager)?)
    }
}

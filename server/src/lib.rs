//! Secrets server

#![deny(missing_docs)]

use std::{fs, net::SocketAddr, path::PathBuf};

use db::DbConn;

mod config;
mod db;
mod service;

use ::service::http::server::HttpServer;
pub use config::*;

/// Server
#[derive(Debug)]
pub struct Server {
    /// Port
    pub port: u16,
    /// Path to the database file (`****.db`)
    pub database: PathBuf,
}

impl Server {
    /// Instantiates a new [Server]
    pub fn new(config: Config) -> Self {
        Server {
            port: config.port,
            database: config.database,
        }
    }

    /// Returns the server address
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.port))
    }

    /// Returns a database connection pool
    pub async fn db(&self) -> anyhow::Result<DbConn> {
        db::conn_pool(&self.database).await
    }
}

impl Server {
    /// Initializes the server
    pub async fn init(&self) -> anyhow::Result<()> {
        if !self.database.exists() {
            fs::write(&self.database, "")?;
        }

        let db_conn = self.db().await?;
        db::init(&db_conn).await?;

        Ok(())
    }

    /// Starts the server
    pub async fn start(self) -> anyhow::Result<()> {
        // Set the db connection
        let db_conn = self.db().await?;

        // Initialize the service
        let service = service::ServiceImpl::new(db_conn);

        // Configure the router
        let addr = self.addr();
        let http_server = HttpServer::new(service, addr);
        http_server.start().await;

        Ok(())
    }
}

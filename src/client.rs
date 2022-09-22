//! Client

use std::str::FromStr;

use reqwest::Url;

mod api;
mod config;

pub use config::*;

/// Client
#[derive(Debug)]
pub struct Client {
    /// Server URL
    pub url: Url,
    /// Authentication token
    pub token: Option<String>,
}

impl Client {
    /// Instantiates a new [Client]
    pub fn new(config: ClientConfig) -> anyhow::Result<Self> {
        let url = Url::from_str(&config.url)?;
        let token = config.token.clone();

        Ok(Self { url, token })
    }
}

//! Client config

//! Server configuration

use std::{fs, path::PathBuf};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

/// App directory
const APP_DIR: &str = "secrets";

/// Configuration file name
const CONFIG_FILE: &str = "client.toml";

/// Client configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Server URL
    pub url: String,
    /// Authentication token
    pub token: Option<String>,
}

impl ClientConfig {
    /// Loads the server configuration file
    ///
    /// Returns [None] if the config file is not found
    pub fn load() -> anyhow::Result<Option<Self>> {
        let cfg_file = config_file()?;
        if !cfg_file.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(&cfg_file)?;
        let cfg: Self = toml::from_str(&data)?;
        Ok(Some(cfg))
    }

    /// Returns the path to the config directory
    pub fn config_dir(&self) -> anyhow::Result<PathBuf> {
        config_dir()
    }

    /// Returns the path to the config directory
    pub fn config_file(&self) -> anyhow::Result<PathBuf> {
        config_file()
    }

    /// Returns as TOML
    pub fn toml(&self) -> anyhow::Result<String> {
        Ok(toml::to_string(self)?)
    }

    /// Saves the file to disk
    pub fn save(&self) -> anyhow::Result<()> {
        let cfg_dir = self.config_dir()?;
        if !cfg_dir.exists() {
            fs::create_dir(&cfg_dir)?;
        }
        let data = toml::to_string(self)?;
        let cfg_file = self.config_file()?;
        Ok(fs::write(&cfg_file, data)?)
    }
}

/// Returns the config dir
fn config_dir() -> anyhow::Result<PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("Config directory not found"))?;
    Ok(config_dir.join(APP_DIR))
}

/// Returns the config file
fn config_file() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join(CONFIG_FILE))
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:6666".to_string(),
            token: None,
        }
    }
}

//! Server commands

use std::{path::PathBuf, str::FromStr};

use anyhow::anyhow;
use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::server::{Server, ServerConfig};

/// Init server CLI arguments
#[derive(Debug, Parser)]
pub struct InitArgs {}

/// Initializes the server
pub async fn init(_args: InitArgs) -> anyhow::Result<()> {
    // Checks if the config exists
    if let Some(cfg) = ServerConfig::load()? {
        eprintln!(
            "{} {}: {}",
            "i".bright_cyan(),
            "Config found".bold(),
            cfg.config_file()?.display()
        );

        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to reset the config?")
            .report(true)
            .interact()?
        {
            return Ok(());
        }
    } else {
        eprintln!("{} No config found", "i".bright_cyan());
    }

    let mut config = ServerConfig::default();

    // Ask for server port
    let input_port: u16 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Server port")
        .default(6666)
        .report(true)
        .interact()?;
    config.port = input_port;

    // Ask for server database path
    let input_db: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Db path")
        .default(config.database.to_str().unwrap().to_string())
        .interact()?;
    config.database = PathBuf::from_str(&input_db)?;

    // Write the config to disk
    config.save()?;
    eprintln!(
        "{} {}: {}",
        "✔".bright_green(),
        "Config saved".bold(),
        config.config_file()?.display()
    );
    eprintln!();
    eprintln!("{}", config.toml()?);

    // Init the server
    let server = Server::new(config);
    server.init().await?;
    eprintln!("{} {}", "✔".bright_green(), "Server initialized".bold());

    Ok(())
}

/// Start server CLI arguments
#[derive(Debug, Parser)]
pub struct StartArgs {}

/// Starts the server
pub async fn start(_args: StartArgs) -> anyhow::Result<()> {
    // Load the configuration
    let config = ServerConfig::load()?.ok_or_else(|| anyhow!("Config not found"))?;

    // Starts the server
    let server = Server::new(config);
    eprintln!(
        "{} {}: {}",
        "✔".bright_green(),
        "Listening on".bold(),
        server.addr()
    );
    server.start().await
}

/// Server info CLI arguments
#[derive(Debug, Parser)]
pub struct InfoArgs {}

/// Returns the server info
pub async fn info(_args: InfoArgs) -> anyhow::Result<()> {
    // Load the configuration
    let config = ServerConfig::load()?.ok_or_else(|| anyhow!("Config not found"))?;

    // Print the server info
    eprintln!("{} {}", "✔".bright_green(), "Server info".bold());
    eprintln!();
    eprintln!("{}", config.toml()?);
    Ok(())
}

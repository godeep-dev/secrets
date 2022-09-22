//! Client commands

use anyhow::anyhow;
use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::client::{Client, ClientConfig};

/// Client init CLI arguments
#[derive(Debug, Parser)]
pub struct InitArgs {}

/// Initializes the server
pub async fn init(_args: InitArgs) -> anyhow::Result<()> {
    // Checks if the config exists
    if let Some(cfg) = ClientConfig::load()? {
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

    let mut config = ClientConfig::default();

    // Ask for server url
    let input_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Server URL")
        .default("http://localhost:6666".to_string())
        .report(true)
        .interact()?;
    config.url = input_url;

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

    // Init the client
    let _client = Client::new(config)?;
    eprintln!("{} {}", "✔".bright_green(), "Client initialized".bold());

    Ok(())
}

/// Status CLI args
#[derive(Debug, Parser)]
pub struct StatusArgs {}

/// Queries the server status
pub async fn status(_args: StatusArgs) -> anyhow::Result<()> {
    // Load the configuration
    let config = ClientConfig::load()?.ok_or_else(|| anyhow!(""))?;
    let url = config.url.clone();

    // Set the client
    let client = Client::new(config)?;
    let _status = client.server_status().await?;
    eprintln!("{} {}", "•".bright_green(), "Server up".bold());
    eprintln!("- url: {}", url);

    Ok(())
}

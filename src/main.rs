use std::process::exit;

use clap::{Parser, Subcommand};
use colored::Colorize;

use secrets::cmd;

/// Secret manager
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Server commands
    Server {
        #[clap(subcommand)]
        commands: ServerCommands,
    },
    /// Init the client
    Init(cmd::client::InitArgs),
    /// Check the server status
    Status(cmd::client::StatusArgs),
    /// View all secrets
    View {},
    /// Adds a secret
    Add {},
    /// Removes a secret
    Rm {},
    /// Manage the organizations
    Orgs {
        #[clap(subcommand)]
        commands: OrgCommands,
    },
    /// Manage the projects
    Proj {
        #[clap(subcommand)]
        commands: ProjCommands,
    },
}

/// Server commands
#[derive(Debug, Subcommand)]
enum ServerCommands {
    /// Initializes the sever
    Init(cmd::server::InitArgs),
    /// Starts the server
    Start(cmd::server::StartArgs),
    /// Server info
    Info(cmd::server::InfoArgs),
}

/// Organization related subcommands
#[derive(Debug, Subcommand)]
enum OrgCommands {
    /// Lists the organizations
    List {},
    /// Adds an organization
    Add {},
    /// Removes an organization
    Rm {},
}

/// Projects related subcommands
#[derive(Debug, Subcommand)]
enum ProjCommands {
    /// Lists the organizations
    List {},
    /// Adds an organization
    Add {},
    /// Removes an organization
    Rm {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    eprintln!();
    let res = match cli.commands {
        Commands::Server { commands } => match commands {
            ServerCommands::Init(args) => cmd::server::init(args).await,
            ServerCommands::Start(args) => cmd::server::start(args).await,
            ServerCommands::Info(args) => cmd::server::info(args).await,
        },
        Commands::Init(args) => cmd::client::init(args).await,
        Commands::Status(args) => cmd::client::status(args).await,
        Commands::View {} => todo!(),
        Commands::Add {} => todo!(),
        Commands::Rm {} => todo!(),
        Commands::Orgs { commands } => todo!(),
        Commands::Proj { commands } => todo!(),
    };

    match res {
        Ok(_) => {}
        Err(err) => {
            eprintln!();
            eprintln!("{}", format!("✗ {}", err).bright_red());
            exit(1);
        }
    }
}

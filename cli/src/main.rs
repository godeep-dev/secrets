//! Secrets CLI

use std::process::exit;

use clap::{Parser, Subcommand};
use colored::Colorize;

mod server;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    eprintln!();
    let res = match cli.commands {
        Commands::Server { commands } => match commands {
            ServerCommands::Init(args) => server::init(args).await,
            ServerCommands::Start(args) => server::start(args).await,
            ServerCommands::Info(args) => server::info(args).await,
        },
        // Commands::Init(args) => cmd::client::init(args).await,
        // Commands::Status(args) => cmd::client::status(args).await,
        // Commands::Auth { commands } => match commands {
        //     AuthCommands::Signup(args) => client::auth::signup(args).await,
        //     AuthCommands::Login(args) => cmd::client::auth::login(args).await,
        //     AuthCommands::Token(args) => todo!(),
        //     AuthCommands::Logout(args) => todo!(),
        // },
        // Commands::View {} => todo!(),
        // Commands::Add {} => todo!(),
        // Commands::Rm {} => todo!(),
        // Commands::Orgs { commands } => todo!(),
        // Commands::Proj { commands } => todo!(),
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

/// Secret manager
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

/// CLI core commands
#[derive(Debug, Subcommand)]
enum Commands {
    /// Server commands
    Server {
        #[clap(subcommand)]
        commands: ServerCommands,
    },
    // /// Init the client
    // Init(cmd::client::InitArgs),
    // /// Check the server status
    // Status(cmd::client::StatusArgs),
    // /// Authentication commands
    // Auth {
    //     #[clap(subcommand)]
    //     commands: AuthCommands,
    // },
    // /// View all secrets
    // View {},
    // /// Adds a secret
    // Add {},
    // /// Removes a secret
    // Rm {},
    // /// Manage the organizations
    // Orgs {
    //     #[clap(subcommand)]
    //     commands: OrgCommands,
    // },
    // /// Manage the projects
    // Proj {
    //     #[clap(subcommand)]
    //     commands: ProjCommands,
    // },
}

/// Server commands
#[derive(Debug, Subcommand)]
enum ServerCommands {
    /// Initializes the sever
    Init(server::InitArgs),
    /// Starts the server
    Start(server::StartArgs),
    /// Server info
    Info(server::InfoArgs),
}

// /// Authentication subcommands
// #[derive(Debug, Subcommand)]
// enum AuthCommands {
//     /// Signup
//     Signup(cmd::client::auth::SignupArgs),
//     /// Login
//     Login(cmd::client::auth::LoginArgs),
//     /// Get the authentication token
//     Token(cmd::client::InitArgs),
//     /// Logout the user
//     Logout(cmd::client::InitArgs),
// }

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

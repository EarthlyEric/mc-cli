use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mc-cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(clap::Args)]
pub struct ServerArgs {
    #[arg(short, long, help = "Target server host")]
    pub target: String,

    #[arg(long, help = "Path to a text file containing a list of target server hosts")]
    pub targets_list: Option<PathBuf>,

    #[arg(short, long, default_value = "25565", help = "Target server port (default: 25565)")]
    pub port: u16,
}


#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "interactive", about = "Starts the interactive mode")]
    Interactive,

    #[command(name = "version", about = "Show mcCLI's version")]
    Version,

    #[command(name = "server", about = "Server management commands")]
    Server(ServerArgs),
}
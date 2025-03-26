use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

mod libs;
use libs::logo::McCliLogo;
use libs::utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "mc-cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "interactive", about = "Starts the interactive mode")]
    Interactive,
    #[command(name = "version", about = "version")]
    Version,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Interactive => {
            utils::print_gradient_text("Welcome to mc-cli", (255, 0, 0), (0, 0, 255), true);
            
        },
        Commands::Version => {
            let logo = McCliLogo::new();
            logo.display();
            print!("{}", "mc-cli version: ".green());
            print!("{}", VERSION.yellow().bold());
        },
    }
}

use clap::Parser;
use owo_colors::OwoColorize;

mod libs;
use libs::logo::McCliLogo;
use libs::utils;

mod scheme;
use scheme::{Cli, Commands};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cli = Cli::parse();

    utils::clear_terminal();
    
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
        Commands::Server(args) => {
            println!("{}: {}", "Target server".green(), args.target);
            println!("{}: {}", "Port".green(), args.port);
        }
    }
}

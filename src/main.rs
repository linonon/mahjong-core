pub mod logic;
#[path = "command/terminal.rs"]
mod terminal;

use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    // Test {
    //     /// lists test values
    //     #[arg(short, long)]
    //     list: bool,
    // },

    /// 使用命令行打牌
    Cli {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Cli {}) => {
            terminal::run();
        }
        None => {}
    }
}

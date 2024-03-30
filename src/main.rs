use std::path::PathBuf;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(author, version, about)]
struct Cli {

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Validate,
    Build,
    Deploy {

        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,
    }
}

fn main() {
    let cli_options = Cli::parse();

    match cli_options.debug {
        0 => println!("Debug is set to 0"),
        1 => println!("Debug is set to 1"),
        2 => println!("Debug is set to 2"),
        _ => println!("Debug is set to unknown"),
    }
    println!("Hello, world!");
}

use clap::Parser;

use self::cli::{Cli, Commands};

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chunk(args) => {
            println!("chunk: {}", args.input.display());
        }
        Commands::Inspect(args) => {
            println!("inspect: {}", args.input.display());
        }
    }
}

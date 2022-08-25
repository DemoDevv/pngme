use clap::Parser;
use crate::args::CommandsWithArgs;
use crate::commands::{encode, decode, remove, print};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod utils;
mod fs;
mod error;

pub use crate::error::{PngError, Result};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {

    #[clap(subcommand)]
    commands: CommandsWithArgs,

}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.commands {
        CommandsWithArgs::Encode(args) => {
            encode(&args.file_path, &args.chunk_type, &args.message, &args.output_file)?;
        },
        CommandsWithArgs::Decode(args) => {
            decode(&args.file_path, &args.chunk_type)?;
        },
        CommandsWithArgs::Remove(args) => {
            remove(&args.file_path, &args.chunk_type)?;
        },
        CommandsWithArgs::Print(args) => {
            print(&args.file_path)?;
        }
    }

    Ok(())
}

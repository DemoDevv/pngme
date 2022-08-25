use clap::{Subcommand, Args};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum CommandsWithArgs {
    /// Encoded a secret message in a PNG file.
    Encode(EncodeArgs),
    /// Decoded a secret message in a PNG file.
    Decode(DecodeArgs),
    /// Removed a secret message in a PNG file.
    Remove(RemoveArgs),
    /// Prints all of the chunks in a PNG file.
    Print(PrintArgs)
}

#[derive(Args)]
pub struct EncodeArgs {

    /// file path of the png file.
    #[clap(value_parser)]
    pub file_path: PathBuf,

    /// type of chunk that will be added.
    #[clap(value_parser)]
    pub chunk_type: String,

    /// message contained in the chunk.
    #[clap(value_parser)]
    pub message: String,

    /// path of the file where you want it to be stored. [Optional]
    #[clap(value_parser)]
    pub output_file: Option<PathBuf>

}

#[derive(Args)]
pub struct DecodeArgs {

    /// file path of the png file.
    #[clap(value_parser)]
    pub file_path: PathBuf,

    /// type of chunk to be decoded.
    #[clap(value_parser)]
    pub chunk_type: String

}

#[derive(Args)]
pub struct RemoveArgs {

    /// file path of the png file.
    #[clap(value_parser)]
    pub file_path: PathBuf,

    /// type of chunk to be removed.
    #[clap(value_parser)]
    pub chunk_type: String

}

#[derive(Args)]
pub struct PrintArgs {

    /// file path of the png file.
    #[clap(value_parser)]
    pub file_path: PathBuf

}
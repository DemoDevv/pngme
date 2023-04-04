use std::path::PathBuf;

use clap::{App, Arg, ArgGroup};
use crate::commands::{encode, decode, remove, print};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod utils;
mod fs;
mod error;
mod download;

pub use crate::error::{PngError, Result};

fn main() -> Result<()> {

    let matches = App::new("pngme")
        .version("0.2.0")
        .author("DemoDevv")
        .about("A tool to encode and decode secret data in PNG files")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(App::new("encode")
            .about("Encode a message in a PNG file")
            .arg(Arg::new("file_path")
                .help("The path to the PNG file")
                .takes_value(true)
                .required_unless("url")
                .long("file_path"))
            .arg(Arg::new("url")
                .help("The url to the PNG file")
                .takes_value(true)
                .required_unless("file_path")
                .long("url"))
            .arg(Arg::new("chunk_type")
                .help("The chunk type to use")
                .required(true)
                .takes_value(true))
            .arg(Arg::new("message")
                .help("The message to encode")
                .required(true)
                .takes_value(true))
            .arg(Arg::new("output_file")
                .help("The path to the output file")
                .required(false)
                .takes_value(true))
            .group(ArgGroup::new("type_path")
                .args(&["file_path", "url"])
                .required(true)))
        .subcommand(App::new("decode")
            .about("Decode a message from a PNG file")
            .arg(Arg::new("file_path")
                .help("The path to the PNG file")
                .takes_value(true)
                .required(true)
                .index(1))
            .arg(Arg::new("chunk_type")
                .help("The chunk type to use")
                .required(true)
                .takes_value(true)
                .index(2)))
        .subcommand(App::new("remove")
            .about("Remove a chunk from a PNG file")
            .arg(Arg::new("file_path")
                .help("The path to the PNG file")
                .takes_value(true)
                .required(true)
                .index(1))
            .arg(Arg::new("chunk_type")
                .help("The chunk type to use")
                .required(true)
                .takes_value(true)
                .index(2)))
        .subcommand(App::new("print")
            .about("Print all of the chunks in a PNG file")
            .arg(Arg::new("file_path")
                .help("The path to the PNG file")
                .takes_value(true)
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        Some(("encode", encode_matches)) => {
            let path: Option<&str> = encode_matches.value_of("file_path");
            let url: Option<&str> = encode_matches.value_of("url");
            let chunk_type: &str = encode_matches.value_of("chunk_type").unwrap();
            let message: &str = encode_matches.value_of("message").unwrap();
            let output_file: Option<PathBuf> = encode_matches.value_of("output_file").map(|s| s.into());
            encode(&path, &url, chunk_type, message, &output_file)?;
        }
        Some(("decode", decode_matches)) => {
            let path: PathBuf = decode_matches.value_of("file_path").unwrap().into();
            decode(&path, decode_matches.value_of("chunk_type").unwrap())?;
        }
        Some(("remove", remove_matches)) => {
            let path: PathBuf = remove_matches.value_of("file_path").unwrap().into();
            remove(&path, remove_matches.value_of("chunk_type").unwrap())?;
        }
        Some(("print", print_matches)) => {
            let path: PathBuf = print_matches.value_of("file_path").unwrap().into();
            print(&path)?;
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    Ok(())
}

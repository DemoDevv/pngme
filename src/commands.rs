use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::download::download_png_from_url;
use crate::fs;
use crate::{PngError, Result};

use std::path::PathBuf;
use std::str::FromStr;

pub fn encode(
    file_path: &Option<&str>,
    url: &Option<&str>,
    chunk_type: &str,
    message: &str,
    output_file: &Option<PathBuf>,
) -> Result<()> {
    let mut png;
    let mut path = &PathBuf::from_str("dist/output.png").unwrap();
    let file_path_buf: PathBuf;

    if let Some(file_path) = file_path {
        file_path_buf = PathBuf::from_str(file_path).unwrap();
        png = fs::file_path_to_png(&file_path_buf)?;
        path = &file_path_buf;
    } else if let Some(url) = url {
        if let None = output_file {
            return Err(PngError::OutputFileNotSpecified);
        }
        png = download_png_from_url(url).unwrap();
    } else {
        return Err(PngError::NoSource);
    }

    let chunk_type = ChunkType::from_str(chunk_type)?;

    png.append_chunk(Chunk::new(chunk_type, message.as_bytes().to_vec()));

    match output_file {
        Some(output_file) => {
            // return the file to the output_file
            fs::vec_to_new_png_file(output_file, &png.as_bytes())
        }
        None => {
            // return the file to the previous file
            fs::rewrite_png_file(path, &png.as_bytes())
        }
    }
}

pub fn decode(file_path: &PathBuf, chunk_type: &str) -> Result<Option<Chunk>> {
    let png = fs::file_path_to_png(file_path)?;
    match png.chunk_by_type(chunk_type) {
        Some(chunk) => {
            println!("{}", chunk);
            Ok(Some(Chunk::new(
                ChunkType::from_str(chunk_type)?,
                chunk.data().to_vec(),
            )))
        }
        None => Err(PngError::ChunkNotFound),
    }
}

pub fn remove(file_path: &PathBuf, chunk_type: &str) -> Result<Option<Chunk>> {
    let mut png = fs::file_path_to_png(file_path)?;
    match png.remove_chunk(chunk_type) {
        Ok(chunk) => {
            println!(
                "chunk with chunk type {} has been deleted",
                chunk.chunk_type()
            );
            fs::rewrite_png_file(file_path, &png.as_bytes())?;
            Ok(Some(chunk))
        }
        Err(error) => Err(error),
    }
}

pub fn print(file_path: &PathBuf) -> Result<()> {
    let png = fs::file_path_to_png(file_path)?;
    println!("{}", png);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "E:/rust/pngme/assets/bg sa mere.png";
    const TEST_DATA: &str = "This is where your secret message will be!";

    // DON'T RUN ALL THESE TESTS AT THE SAME TIME !
    // USE TEST ONE BY ONE !
    // thanks <3

    #[test]
    // pub fn test_encode_command_with_local_file_for_data() {
    //     encode(
    //         &Some(PathBuf::from(TEST_FILE_PATH)),
    //         &None,
    //         &String::from("RuSt"),
    //         &String::from(TEST_DATA),
    //         &None,
    //     )
    //     .unwrap();
    //     let png = fs::file_path_to_png(&PathBuf::from(TEST_FILE_PATH)).unwrap();
    //     let chunks = png.chunks();
    //     assert_eq!(
    //         TEST_DATA,
    //         &chunks[chunks.len() - 1].data_as_string().unwrap()
    //     );
    // }

    #[test]
    pub fn test_decode_command() {
        assert!(decode(&PathBuf::from(TEST_FILE_PATH), &String::from("RuSt")).is_ok());
    }

    #[test]
    pub fn test_remove_command() {
        assert!(remove(&PathBuf::from(TEST_FILE_PATH), &String::from("RuSt")).is_ok());
    }
}

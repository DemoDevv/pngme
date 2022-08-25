use crate::{Result, PngError};
use crate::fs;
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

use std::path::PathBuf;
use std::str::FromStr;

fn file_path_to_png(file_path: &PathBuf) -> Result<Png> {
    let file_in_vec = fs::png_file_to_vec(file_path)?;
    let png = Png::try_from(file_in_vec.as_ref())?;
    Ok(png)
}

pub fn encode(file_path: &PathBuf, chunk_type: &String, message: &String, output_file: &Option<PathBuf>) -> Result<()> {
    let mut png = file_path_to_png(file_path)?;
    let chunk_type = ChunkType::from_str(chunk_type)?;
    
    png.append_chunk(Chunk::new(chunk_type, message.as_bytes().to_vec()));

    match output_file {
        Some(output_file) => {
            // return the file to the output_file
            fs::vec_to_new_png_file(output_file, &png.as_bytes())
        },
        None => {
            // return the file to the previous file
            fs::rewrite_png_file(file_path, &png.as_bytes())
        }
    }
}

pub fn decode(file_path: &PathBuf, chunk_type: &String) -> Result<Option<Chunk>> {
    let png = file_path_to_png(file_path)?;
    match png.chunk_by_type(chunk_type) {
        Some(chunk) => {
            println!("{}", chunk);
            Ok(Some(Chunk::new(ChunkType::from_str(chunk_type)?, chunk.data().to_vec())))
        },
        None => {
            Err(PngError::ChunkNotFound)
        }
    }
}

pub fn remove(file_path: &PathBuf, chunk_type: &String) -> Result<Option<Chunk>> {
    let mut png = file_path_to_png(file_path)?;
    match png.remove_chunk(chunk_type) {
        Ok(chunk) => {
            println!("chunk with chunk type {} has been deleted", chunk.chunk_type());
            fs::rewrite_png_file(file_path, &png.as_bytes())?;
            Ok(Some(chunk))
        },
        Err(error) => {
            Err(error)
        }
    }
}

pub fn print(file_path: &PathBuf) -> Result<()> {
    let png = file_path_to_png(file_path)?;
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
    pub fn test_encode_command_for_data() {
        encode(&PathBuf::from(TEST_FILE_PATH), &String::from("RuSt"), &String::from(TEST_DATA), &None).unwrap();
        let png = file_path_to_png(&PathBuf::from(TEST_FILE_PATH)).unwrap();
        let chunks = png.chunks();
        assert_eq!(TEST_DATA, &chunks[chunks.len() - 1].data_as_string().unwrap());
    }

    #[test]
    pub fn test_decode_command() {
        assert!(decode(&PathBuf::from(TEST_FILE_PATH), &String::from("RuSt")).is_ok());
    }

    #[test]
    pub fn test_remove_command() {
        assert!(remove(&PathBuf::from(TEST_FILE_PATH), &String::from("RuSt")).is_ok());
    }
}

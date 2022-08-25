pub use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

use crate::Result;

pub fn png_file_to_vec(file_path: &PathBuf) -> Result<Vec<u8>> {
    Ok(fs::read(file_path)?)
}

pub fn vec_to_new_png_file(file_path: &PathBuf, contents: &Vec<u8>) -> Result<()> {
    Ok(fs::write(file_path, contents)?)
}

pub fn rewrite_png_file(file_path: &PathBuf, contents: &Vec<u8>) -> Result<()> {
    vec_to_new_png_file(file_path, contents)
}

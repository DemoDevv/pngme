pub use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

use crate::Result;
use crate::png::Png;

pub fn png_file_to_vec(file_path: &PathBuf) -> Result<Vec<u8>> {
    Ok(fs::read(file_path)?)
}

pub fn vec_to_new_png_file(file_path: &PathBuf, contents: &Vec<u8>) -> Result<()> {
    Ok(fs::write(file_path, contents)?)
}

pub fn file_path_to_png(file_path: &PathBuf) -> Result<Png> {
    let file_in_vec = png_file_to_vec(file_path)?;
    let png = Png::try_from(file_in_vec.as_ref())?;
    Ok(png)
}

pub fn rewrite_png_file(file_path: &PathBuf, contents: &Vec<u8>) -> Result<()> {
    vec_to_new_png_file(file_path, contents)
}

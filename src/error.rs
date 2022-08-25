use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, PngError>;

#[derive(Debug)]
pub enum PngError {
    ChunkNotFound,
    InvalidPngFile,
    InvalidCrc,
    EmptyCrc,
    InvalidSegment,
    ShortChunk,
    ChunkDataUTF8Error,
    FileNotFound,
}

use PngError::*;

impl std::error::Error for PngError {}

impl Display for PngError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let error_message = match self {
            ChunkNotFound => "Chunk not found",
            InvalidPngFile => "Invalid png file",
            InvalidCrc => "Invalid crc",
            EmptyCrc => "Empty crc",
            InvalidSegment => "Invalid segment",
            ShortChunk => "Short chunk",
            ChunkDataUTF8Error => "Chunk data is not UTF-8",
            FileNotFound => "File not found",
        };

        write!(f, "{}", error_message)
    }
}

impl From<std::io::Error> for PngError {
    fn from(_: std::io::Error) -> Self {
        PngError::FileNotFound
    }
}

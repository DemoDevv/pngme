use std::io::{BufReader, Read};

use crate::{PngError, Result};

pub fn ref_buffer_to_u32(buf: &[u8]) -> u32 {
    let mut buf_reader = BufReader::new(buf);
    let mut buffer: [u8; 4] = [0, 0, 0, 0];

    buf_reader.read_exact(&mut buffer).unwrap();
    u32::from_be_bytes(buffer)
}

pub fn segment4(bytes: &[u8]) -> Result<[u8; 4]> {
    bytes.try_into().map_err(|_| PngError::InvalidSegment)
}

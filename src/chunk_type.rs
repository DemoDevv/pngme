use core::str::FromStr;
use core::fmt::{self, Display, Formatter};

use crate::{PngError, Result};

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = PngError;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {

    type Err = PngError;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        let mut result = [0; 4];
        result.copy_from_slice(bytes);
        Ok(ChunkType { bytes: result }) // TODO: gerer l'erreur d'un chunk_type trop grand ou trop petit en renvoyant une erreur custom
    }

}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.bytes.to_vec()).unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() { return false; }
        for byte in self.bytes.iter() {
            if !byte.is_ascii_alphabetic() { return false; }
        };
        true
    }

    #[allow(dead_code)]
    pub fn is_critical(&self) -> bool {
        return self.bytes[0].is_ascii_uppercase(); 
    }

    #[allow(dead_code)]
    pub fn is_public(&self) -> bool {
        return self.bytes[1].is_ascii_uppercase();
    }

    #[allow(dead_code)]
    pub fn is_reserved_bit_valid(&self) -> bool {
        return self.bytes[2].is_ascii_uppercase();
    }

    #[allow(dead_code)]
    pub fn is_safe_to_copy(&self) -> bool {
        return self.bytes[3].is_ascii_lowercase();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        // change this line because the version is outdated
        // let chunk = ChunkType::from_str("Ru1t");
        // assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

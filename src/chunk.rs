use crate::{PngError, Result};
use crate::chunk_type::ChunkType;
use core::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = PngError;

    fn try_from(value: &[u8]) -> Result<Self> {
        let chunk_type_bytes: [u8; 4] = value[4..8].try_into().unwrap();
        let main_data_bytes: Vec<u8> = value[8..value.len() - 4].to_vec();
        let crc_bytes: &[u8] = &value[value.len() - 4..value.len()];

        let chunk_type = ChunkType::try_from(chunk_type_bytes).unwrap();

        if crc_bytes.is_empty() {
            return Err(PngError::EmptyCrc);
        }

        let chunk = Chunk::new(chunk_type, main_data_bytes);

        let data_lenght = crate::utils::ref_buffer_to_u32(crc_bytes);

        if chunk.crc() != data_lenght {
            return Err(PngError::InvalidCrc);
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "chunk_type: {}\n\r", self.chunk_type)?;
        write!(f, "length: {}\n\r", self.length())?;
        match self.data_as_string() {
            Ok(data) => write!(f, "data: {}\n\r", data)?,
            Err(_) => write!(f, "data: {}\n\r", "None")?
        };
        write!(f, "crc: {}\n\r", self.crc())?;
        write!(f, "bytes: {:?}\n\r", self.as_bytes())
    }
}

impl Chunk {

    pub const CHUNK_METADATA_LENGTH: usize = 12;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk {
            chunk_type,
            data,
        }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        let data = self.data();
        let type_as_bytes = self.chunk_type.bytes();
        let total_bytes: Vec<u8> = type_as_bytes.iter().chain(data.iter()).cloned().collect();

        crc::crc32::checksum_ieee(&total_bytes)
    }

    pub fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => Ok(s),
            Err(_) => Err(PngError::ChunkDataUTF8Error),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_as_bytes: Vec<u8> = self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect();
        chunk_as_bytes
    }
}

pub struct ChunkIterator<'a> {
    cur: &'a [u8],
    tainted: bool,
}

impl<'a> ChunkIterator<'a> {
    pub fn new(chunks: &'a [u8]) -> ChunkIterator {
        ChunkIterator { cur: chunks, tainted: false }
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Result<Chunk>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tainted || self.cur.len() == 0 { // tainted or empty
            return None; // no more chunks
        } else if self.cur.len() < Chunk::CHUNK_METADATA_LENGTH { // short chunk
            self.tainted = true;
            return Some(Err(PngError::ShortChunk));
        }

        let len = super::utils::segment4(&self.cur[0..4]).unwrap(); // recuperer l'octet de la longueur du chunk
        let len = u32::from_be_bytes(len) as usize + Chunk::CHUNK_METADATA_LENGTH; // transformer la longueur en entier en ajoutant la longueur des metadatas

        if self.cur.len() < len { // verifier si y a encore assez de donnees pour un chunk
            self.tainted = true;
            return Some(Err(PngError::ShortChunk));
        }

        let chunk = &self.cur[0..len]; // recuperer le chunk
        self.cur = &self.cur[len..]; // supprimer le chunk de la liste des chunks

        Chunk::try_from(chunk).map_or_else( // convertir le chunk en objet Chunk
            |e| { // si erreur
                self.tainted = true; // marquer le chunk comme tainted
                Some(Err(e)) // retourner l'erreur
            },
            |chunk| Some(Ok(chunk)), // sinon retourner le chunk
        )    
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);

        println!("{}", chunk);

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();

        println!("{}", chunk);

        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();

        println!("{}", chunk);

        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        println!("{}", chunk);

        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();

        println!("{}", chunk);

        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        println!("{}", chunk);

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        println!("{}", chunk);
        
        let _chunk_string = format!("{}", chunk);
    }
}

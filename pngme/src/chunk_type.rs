
use std::{fmt::{self, Display}, str::{from_utf8, FromStr}};

use super::{Result, Error};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    data : u32,
}

impl TryFrom<[u8;4]> for ChunkType {

    type Error = Error;

    fn try_from(value: [u8;4]) -> Result<Self> {
        
        let mut new_chunk_type= ChunkType {
            data : 0
        };

        for &i in value.iter() {
            if !i.is_ascii_alphabetic() {
                return Err(Box::new(fmt::Error)); 
            }    
        }


        new_chunk_type.data = (value[0] as u32) << 24 | (value[1] as u32) << 16 
        | (value[2] as u32) << 8 | (value[3] as u32);


        Ok(new_chunk_type)
    }
}

impl FromStr for ChunkType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        
        let bytes = s.as_bytes();

        let mut data = [0,0,0,0];

        for i in 0..4 {
            data[i] = bytes[i];
        }

        Ok(ChunkType::try_from(data)?)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.data.to_be_bytes();
        
        write!(f, "{}", from_utf8(&bytes).unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.data.to_be_bytes()
    }

    pub fn is_valid(&self) -> bool {

        let bytes = self.data.to_be_bytes();

        for i in 0..4 {
            if (bytes[i].is_ascii_lowercase())   || (bytes[i].is_ascii_uppercase()) {
                    if i == 2 && bytes[i].is_ascii_lowercase() {
                        return false;
                    }
                    
                    continue;
               }
            return false;   
        }
        true
    }

    pub fn is_critical(&self) -> bool {
        return self.data.to_be_bytes()[0].is_ascii_uppercase();
    }


    pub fn is_public(&self) -> bool {
        return self.data.to_be_bytes()[1].is_ascii_uppercase();
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        return self.data.to_be_bytes()[2].is_ascii_uppercase();
    }

    pub fn is_safe_to_copy(&self) -> bool {
        return self.data.to_be_bytes()[3].is_ascii_lowercase();
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

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
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
use super::{chunk_type::ChunkType, utils, Error, Result, error::DataError};
use crc::{Crc, CRC_32_ISO_HDLC};
use std::{
    fmt::{self, Display},
    str::from_utf8,
};


#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}


impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {

        let length = utils::u8_array_to_u32(
            value[0..4]
                .try_into()
                .map_err(|_| "Slice is not of length 4")?);

        if value.len() < 12 + length as usize{
            dbg!(value.len(), length);
            return Err(Box::new(DataError));
        }    

        let res = Chunk {
            length,
            chunk_type: {
                let bytes: [u8; 4] = value[4..8]
                    .try_into()
                    .map_err(|_| "Slice is not of length 4")?;
                ChunkType::try_from(bytes)?
            },
            data: value[8..8+length as usize].to_vec(),
            crc: utils::u8_array_to_u32(
                value[8+ length as usize.. 12 + length as usize]
                    .try_into()
                    .map_err(|_| "Slice is not of length 4")?,
            ),
        };

        let mut temp_vec = Vec::new();
        temp_vec.extend(res.chunk_type.bytes());
        temp_vec.extend(&res.data);

        if Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(temp_vec.as_slice()) != res.crc {
             dbg!(res.crc);
             return Err(Box::new(DataError));
        }
        Ok(res)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 将字节数组转换为 &str，并处理可能的错误
        match from_utf8(&self.as_bytes()) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => {
                dbg!();
                write!(f, "[Invalid UTF-8]")
            }
        }
    }
}

impl Chunk {


    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let len = data.len();
        let mut temp = Vec::new();
        temp.extend(chunk_type.bytes());
        temp.extend(&data);
        Chunk {
            length: (len as u32),
            chunk_type,
            data,
            crc: Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(temp.as_slice()),
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        // Convert the Vec<u8> to a String
        let string_result = String::from_utf8(self.data.clone());

        // Handle potential errors and return the Result
        match string_result {
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.length.to_be_bytes());
        bytes.extend(&self.chunk_type.bytes());
        bytes.extend(&self.data);
        bytes.extend(&self.crc.to_be_bytes());
        bytes
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
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
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
        
        let _chunk_string = format!("{}", chunk);
    }
}
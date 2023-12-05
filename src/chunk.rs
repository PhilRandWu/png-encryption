use std::convert::TryInto;
use std::io::Read;
use std::fmt::Display;
// fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk
// fn length(&self) -> u32
// fn chunk_type(&self) -> &ChunkType
// fn data(&self) -> &[u8]
// fn crc(&self) -> u32
// fn data_as_string(&self) -> Result<String>
// fn as_bytes(&self) -> Vec<u8>
// //还要 TryFrom<&[u8]>, Display
use crate::chunk_type::ChunkType;
#[derive(Debug)]
pub struct Chunk {
    // 一个 4 字节无符号整数，表示块数据字段中的字节数。长度仅计算数据字段，而不计算其本身、块类型代码或 CRC。
    // 零是有效长度。尽管编码器和解码器应将长度视为无符号，但其值不得超过 231 字节
    length: u32,
    // 一个 4 字节的块类型代码。为了便于描述和检查 PNG 文件，类型代码仅限于由大写和小写 ASCII 字母（A-Z 和 a-z，或 65-90 和 97-122）组成。
    // 但是，编码器和解码器必须将代码视为固定的二进制值，而不是字符串(也就是要看作一个一个字节)
    chunk_type: ChunkType,
    // 适合块类型的数据字节。该字段可以是零长度
    data: Vec<u8>,
    // 一个4字节的CRC（Cyclic Redundancy Check）是根据chunk前面的字节计算出来的，
    // 包括chunk type和chunk data字段，但不包括length字段。 CRC 始终存在，即使对于不包含数据的块也是如此
    crc: u32,
}


impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let crc_content = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<u8>>();
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&crc_content);
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let string = String::from_utf8(self.data.clone())?;
        Ok(string)
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

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err("Chunk data is too short");
        }
        let chunk_type_bytes: [u8; 4] = bytes[4..8].try_into().unwrap(); // 4 ~ 7
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;
        let data = bytes[8..bytes.len() -4 ].to_vec(); // 8 ~ -4
        let crc_bytes: [u8; 4] = bytes[bytes.len() - 4..].try_into().unwrap(); // -4 ~ ..
        let crc = u32::from_be_bytes(crc_bytes);
        let chunk = Chunk::new(chunk_type, data);
        println!("{} - {}", chunk.crc, crc);
        if chunk.crc != crc {
            return Err("CRC does not match");
        }
        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chunk_string = self
            .data_as_string()
            .unwrap_or_else(|_| String::from("Invalid UTF-8"));
        write!(
            f,
            "Chunk\n\tLength: {}\n\tType: {}\n\tData: {}\n\tCRC: {}",
            self.length, self.chunk_type, chunk_string, self.crc
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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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



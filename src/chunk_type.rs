// fn bytes(&self) -> [u8; 4]
// fn is_valid(&self) -> bool
// fn is_critical(&self) -> bool
// fn is_public(&self) -> bool
// fn is_reserved_bit_valid(&self) -> bool
// fn is_safe_to_copy(&self) -> bool
// 还要实现 TryFrom<[u8; 4]>, FromStr, Display, PartialEq 和 Eq

use std::convert::TryFrom;
use std::{fmt::{Display, Formatter}, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    first_byte: u8,
    // 遇到辅助位为 1 的未知块的解码器可以安全地忽略该块并继续显示图像,比如时间块 (tIME)
    second_byte: u8,
    // 公共块是 PNG 规范的一部分或在 PNG 专用公共块类型列表中注册的块。应用程序还可以为自己的目的定义私有（未注册）块
    third_byte: u8,
    // 第三个字母大小写的意义是为将来可能的扩展保留的。目前所有的块名都必须有大写(0)的第三个字母
    fourth_byte: u8, // 如果块的安全复制位为 1，则无论软件是否识别块类型，也不管文件修改的范围如何，都可以将块复制到修改后的 PNG 文件
}


impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.first_byte,
            self.second_byte,
            self.third_byte,
            self.fourth_byte
        ]
    }

    /**
    该方法的返回值表示 self.first_byte 的第 6 位是否为 0。
     */
    pub fn is_critical(&self) -> bool {
        self.first_byte & 0b0010_0000 == 0
    }

    /**
    是否是一个公开的块
     */
    pub fn is_public(&self) -> bool {
        self.second_byte & 0b0010_0000 == 0
    }
    /**
    目前所有的块名都必须有大写(0)的第三个字母,否则无效
     */
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.third_byte & 0b0010_0000 == 0
    }

    /**
    可保证无论如何能将块进行复制
     */
    pub fn is_safe_to_copy(&self) -> bool {
        self.fourth_byte & 0b0010_0000 != 0
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}


impl TryFrom<[u8; 4]> for ChunkType {
    //! TryFrom 是 Rust 语言中的一个 trait，用于将一种类型转换为另一种类型。
    type Error = &'static str;
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if !is_all_alphabetic(bytes) {
            return Err("ChunkType must be alphabetic");
        }
        let chunk_type = ChunkType {
            first_byte: bytes[0],
            second_byte: bytes[1],
            third_byte: bytes[2],
            fourth_byte: bytes[3],
        };
        Ok(chunk_type)
    }
}

impl FromStr for ChunkType {
    //! FromStr 是 Rust 中的一个 trait，用于从字符串类型转换为目标类型。
    //! &str -> bytes -> TryForm try_from -> ChunkType
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("ChunkType string must be 4 characters long");
        }
        let mut bytes = [0; 4];
        for (i, byte) in s.bytes().enumerate() {
            bytes[i] = byte;
        }
        Self::try_from(bytes)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bytes = self.bytes();
        for byte in bytes.iter() {
            write!(f, "{}", *byte as char)?;
        }
        Ok(())
    }
}

/**
用于检查一个长度为 4 的字节数组中的所有字节是否都是 ASCII 字母。
 */
fn is_all_alphabetic(bytes: [u8; 4]) -> bool {
    println!("{}", bytes.iter().all(|byte| byte.is_ascii_alphabetic()));
    bytes.iter().all(|byte| byte.is_ascii_alphabetic())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116]; // [u8:4]
        let actual = ChunkType::try_from(expected).unwrap(); // ChunkType
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
        println!("{chunk} - {}", &chunk.to_string());
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
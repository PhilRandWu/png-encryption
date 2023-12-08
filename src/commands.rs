use std::fs;
use std::str::FromStr;
use crate::args::*;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(args: EncodeArgs) -> Result<(), Box<dyn std::error::Error>> {
    if !args.file_path.exists() {
        return Err(Box::try_from("File path is not exist!").unwrap());
    }
    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&bytes[..])?;
    let encode_chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.as_bytes().to_vec(),
    );
    png.append_chunk(encode_chunk);
    if let Some(output) = args.output {
        fs::write(output, png.as_bytes())?
    } else {
        fs::write(args.file_path, png.as_bytes())?
    }
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<(), Box<dyn std::error::Error>> {
    if !args.file_path.exists() {
        return Err(Box::try_from("File path is not exist!").unwrap());
    }
    let bytes = fs::read(args.file_path)?;
    let png = Png::try_from(&bytes[..])?;
    let chunk = png
        .chunks()
        .iter()
        .find(|chunk| chunk.chunk_type().to_string() == args.chunk_type)
        .ok_or("Chunk no find");
    let message: String = chunk?.data().iter().map(|x| *x as char).collect();
    println!("encode message is {message}");
    Ok(())
}


pub fn remove(args: RemoveArgs) -> Result<(), Box<dyn std::error::Error>> {
    if !args.file_path.exists() {
        return Err(Box::try_from("File path is not exist!").unwrap());
    }
    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&bytes[..])?;
    png.remove_chunk(&args.chunk_type)?;
    fs::write(args.file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<(), Box<dyn std::error::Error>> {
    // check if file exists
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path)?;
    let png = Png::try_from(&bytes[..])?;
    // print chunks
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
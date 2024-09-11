use std::convert::TryFrom;
use std::fs;
use std::io::{Read, Write};
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;
use crate::{chunk, chunk_type};
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut file = fs::File::open(&args.file_path)?;

    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    let mut png = Png::try_from(&contents[..])?;

    let chunk_type = chunk_type::ChunkType::from_str(&args.chunk_type)?;

    let message_bytes = args.message.as_bytes().to_vec();

    png.append_chunk(chunk::Chunk::new(chunk_type, message_bytes));

    if let Some(out_file_path) = args.output_file_path {
        let mut file = fs::File::create(&out_file_path)?;
        file.write(&png.as_bytes())?;
    } else {
        let mut file = fs::File::create(&args.file_path)?;
        file.write(&png.as_bytes())?;
    }

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {

    let mut file = fs::File::open(args.file_path)?;

    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    let png = Png::try_from(&contents[..])?;

    if let Some(res) = png.chunk_by_type(&args.chunk_type) {
        println!("{}", res.data_as_string()?);
    }


    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut file = fs::File::open(&args.file_path)?;

    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    let mut png = Png::try_from(&contents[..])?;

    png.remove_first_chunk(&args.chunk_type)?;

    let mut file = fs::File::create(&args.file_path)?;

    file.write(&png.as_bytes())?;

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    
    let mut file = fs::File::open(&args.file_path)?;

    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    let png = Png::try_from(&contents[..])?;

    let chunks = png.chunks();

    for chunk in chunks {
        if let Ok(out_str) = chunk.data_as_string() {
            println!("{}", out_str);
        } else {
            continue;
        }
    }

    Ok(())
}

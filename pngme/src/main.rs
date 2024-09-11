use clap::Parser;
use args::PngMeArgs;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;


mod utils;
mod error;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;



fn main() -> Result<()> {

    
    let command = args::PngMeCommand::parse();
    match command.args {
        PngMeArgs::Encode(encode_args) => {
            commands::encode(encode_args)?;
        }
        PngMeArgs::Decode(decode_args) => {
            commands::decode(decode_args)?;
        }
        PngMeArgs::Remove(remove_args) => {
            commands::remove(remove_args)?;
        }
        PngMeArgs::Print(print_args) => {
            commands::print_chunks(print_args)?;
        }
    }
    Ok(())
}

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Main arguments for the program
#[derive(Parser, Debug)]
#[command(name = "pngme")]
pub struct PngMeCommand {
    #[command(subcommand)]
    pub args : PngMeArgs,
}

/// Subcommands for the program
#[derive(Subcommand, Debug)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    /// Input file path
    pub file_path: PathBuf,

    /// Chunk type
    pub chunk_type: String,

    /// Message to encode
    pub message: String,

    /// Output file path
    pub output_file_path: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    /// Input file path
    pub file_path: PathBuf,

    /// Chunk type
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    /// Input file path
    pub file_path: PathBuf,

    /// Chunk type
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    /// Input file path
    pub file_path: PathBuf,
}

use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "pngEnc")]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: PngMeArgs,
}

#[derive(Debug, Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}

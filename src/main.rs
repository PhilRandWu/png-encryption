use std::env::args;
use clap::Parser;
use crate::args::PngMeArgs;

mod args;
// mod chunk;
mod chunk_type;
mod png;
mod chunk;
mod commands;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Cli::parse();
    println!("{:?}", args);
    match args.subcommand {
        args::PngMeArgs::Encode(encode_args) => commands::encode(encode_args),
        PngMeArgs::Decode(encode_args) => commands::decode(encode_args),
        args::PngMeArgs::Remove(remove_args) => commands::remove(remove_args),
        args::PngMeArgs::Print(print_args) => commands::print(print_args),
    }
}


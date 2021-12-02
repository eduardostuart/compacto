extern crate clap;

use byte_unit::{AdjustedByte, ByteUnit};
use clap::Parser;
use cli::Cli;
use std::{
    fs::{self, File},
    io::Write,
    process,
};

mod cli;

fn main() -> compacto::Result<()> {
    let matches: Cli = Cli::parse();

    let input = match fs::read_to_string(&matches.input) {
        Ok(input) => input,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => eprintln!("File not found: \"{}\"", &matches.input),
                _ => eprintln!("Could not read file: \"{}\"", &matches.input),
            };
            process::exit(1);
        }
    };

    let result = match matches.mode {
        cli::Mode::Compress => compacto::compress_json(&input)?,
        cli::Mode::Decompress => compacto::decompress_json(&input)?,
    };

    let mut output = File::create(&matches.output).unwrap();
    output.write_all(result.as_bytes()).unwrap();

    println!(
        "{}, Size: {}\n{}, Size: {}",
        matches.input,
        get_file_size(&matches.input),
        &matches.output,
        get_file_size(&matches.output)
    );
    Ok(())
}

fn get_file_size(file: &str) -> AdjustedByte {
    let size = File::open(file).unwrap().metadata().unwrap().len().into();
    byte_unit::Byte::from_bytes(size).get_adjusted_unit(ByteUnit::KB)
}

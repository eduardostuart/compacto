extern crate clap;

use byte_unit::{AdjustedByte, ByteUnit};
use clap::Parser;
use cli::Cli;
use std::{
    fs::{self, File},
    io::Write,
};

mod cli;

fn main() -> compacto::Result<()> {
    let matches: Cli = Cli::parse();

    let input_path = matches.input;
    let output_path = matches.output;

    match fs::read_to_string(input_path.clone()) {
        Ok(input) => {
            let result = if matches.compress {
                compacto::compress_json(&input)?
            } else if matches.decompress {
                compacto::decompress_json(&input)?
            } else {
                eprintln!("Invalid mode. Use -c to compress or -d to decompress");
                std::process::exit(1);
            };

            let mut output = File::create(output_path.clone()).unwrap();
            output.write_all(result.to_string().as_bytes()).unwrap();

            let (input_size, output_size) = compare_sizes(&input_path, &output_path);

            println!("{}\nsize: {}", input_path, input_size);
            println!("{}\nsize: {}", output_path, output_size);
        }
        Err(e) => {
            eprintln!("Invalid file: {}", e);
        }
    };

    Ok(())
}

fn compare_sizes(file_a: &str, file_b: &str) -> (AdjustedByte, AdjustedByte) {
    let get_file_size =
        |input: &str| -> u128 { File::open(input).unwrap().metadata().unwrap().len().into() };

    let size_a = get_file_size(file_a);
    let size_b = get_file_size(file_b);

    (
        byte_unit::Byte::from_bytes(size_a).get_adjusted_unit(ByteUnit::KB),
        byte_unit::Byte::from_bytes(size_b).get_adjusted_unit(ByteUnit::KB),
    )
}

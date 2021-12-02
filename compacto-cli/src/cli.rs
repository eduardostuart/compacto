use std::str::FromStr;

use clap::{crate_version, AppSettings, Parser};

#[derive(Clone, Parser, Debug)]
pub enum Mode {
    Compress,
    Decompress,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "compress" => Ok(Mode::Compress),
            "decompress" => Ok(Mode::Decompress),
            _ => Err("no match"),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version = crate_version!(), author = "Eduardo Stuart <e@s.tuart.me>", setting = AppSettings::ArgRequiredElseHelp)]
pub struct Cli {
    pub input: String,
    pub output: String,
    #[clap(multiple_occurrences = false, long, short)]
    pub mode: Mode,
}

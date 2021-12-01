use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Eduardo Stuart <e@s.tuart.me>")]
pub struct Cli {
    pub input: String,
    pub output: String,
    #[clap(short = 'c', long = "compress")]
    pub compress: bool,
    #[clap(short = 'd', long = "decompress")]
    pub decompress: bool,
}

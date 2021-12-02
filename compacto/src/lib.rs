mod compress;
mod decompress;
pub mod error;

/// Alias for a `Result` with the error type `compacto::error::Error`.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

pub use compress::{compress_json, Compressor};
pub use decompress::{decompress_json, Decompressor};

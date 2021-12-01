mod compress;
mod decompress;
pub mod error;

/// Alias for a `Result` with the error type `compacto::error::Error`.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

pub use compress::{compress_json, Compressor};
pub use decompress::{decompress_json, Decompressor};

#[cfg(test)]
mod test_utils {
    use serde_json::Value;

    pub fn get_json_value_sample(file: &str) -> Value {
        let file_path = format!("../samples/test-samples/{}", file);
        let data = std::fs::read_to_string(file_path).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}

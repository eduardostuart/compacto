use std::{error::Error as StdError, fmt, io};

#[derive(Debug)]
pub enum Error {
    /// Represents any IO error
    IO(io::Error),
    /// Representes any JSON deserialization or serialization errors
    JSONError(serde_json::error::Error),
    ///
    UnknownJSONValueRef(serde_json::Value),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Self::JSONError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IO(ref e) => write!(f, "{}", e),
            Self::JSONError(ref e) => write!(f, "{}", e),
            Self::UnknownJSONValueRef(ref e) => write!(f, "{}", e),
        }
    }
}

impl StdError for Error {}

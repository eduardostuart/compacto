use std::{
    error::Error as StdError,
    fmt::{self, Display},
    io,
};

#[derive(Debug)]
pub enum Error {
    /// Represents any IO error
    IO(io::ErrorKind),
    /// Representes any JSON deserialization or serialization errors
    JSONError(String),
    ///
    UnknownJSONValueRef(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e.kind())
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Self::JSONError(e.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IO(ref e) => write!(f, "{:?}", e),
            Self::JSONError(ref e) => write!(f, "{}", e),
            Self::UnknownJSONValueRef(ref e) => write!(f, "{}", e),
        }
    }
}

impl StdError for Error {}

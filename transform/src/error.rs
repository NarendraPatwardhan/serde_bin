use std::fmt;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Unimplemented,
    InvalidData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::Unimplemented => write!(f, "Unsupported type"),
            Error::InvalidData => write!(f, "Invalid data"),
        }
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Custom(msg)
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

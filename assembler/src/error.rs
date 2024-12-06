use std::fmt::{Display, Formatter};
use std::io;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    IO(io::Error)
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IO(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "Input/output error: {e}")
        }
    }
}

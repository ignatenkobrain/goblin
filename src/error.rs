//! A custom Goblin error
//!

use scroll;
use core::result;
use core::fmt::{self, Display};
use std::error;
use std::io;

#[derive(Debug)]
/// A custom Goblin error
pub enum Error {
    /// The binary is malformed somehow
    Malformed(String),
    /// The binary's magic is unknown or bad
    BadMagic(u64),
    /// An error emanating from reading and interpreting bytes
    Scroll(scroll::Error),
    /// An IO based error
    IO(io::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(_) => { "IO error" }
            Error::Scroll(_) => { "Scroll error" }
            Error::BadMagic(_) => { "Invalid magic number" }
            Error::Malformed(_) => { "Entity is malformed in some way" }
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IO(ref io) => { io.cause() }
            Error::Scroll(ref scroll) => { scroll.cause() }
            Error::BadMagic(_) => { None }
            Error::Malformed(_) => { None }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<scroll::Error> for Error {
    fn from(err: scroll::Error) -> Error {
        Error::Scroll(err)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref err) => { write!(fmt, "{}", err) },
            Error::Scroll(ref err) => { write!(fmt, "{}", err) },
            Error::BadMagic(magic) => { write! (fmt, "Invalid magic number: 0x{:x}", magic) },
            Error::Malformed(ref msg) => { write! (fmt, "Malformed entity: {}", msg) },
        }
    }
}

/// An impish result
pub type Result<T> = result::Result<T, Error>;

use std::io;
use std::error;
use std::fmt;

use crate::{KsDevErr};
use crate::format::{DTError, ParseError};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Dev(KsDevErr),
    Parse(ParseError),
    DataType(DTError),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

macro_rules! impl_from {
    ($src:ty, $opt:ident) => {
        impl From<$src> for Error {
            fn from(err: $src) -> Error {
                Error::$opt(err)
            }
        }
    };
}

impl_from!(io::Error, Io);
impl_from!(KsDevErr, Dev);
impl_from!(ParseError, Parse);
impl_from!(DTError, DataType);
impl_from!(String, Other);
impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error::Other(String::from(err))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

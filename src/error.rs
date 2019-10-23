use bincode::Error as SedesError;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::io::Error as IOError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;


#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    IOError(IOError),
    SedesError(SedesError),
    Parse(&'static str),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Network(network_err) => network_err.source(),
            Error::Parse(_) => None,
            Error::IOError(io_err) => io_err.source(),
            Error::SedesError(sedes_err) => sedes_err.source(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error {
    pub fn from_parse(reason: &'static str) -> Error {
        Error::Parse(reason)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Network(error)
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IOError(error)
    }
}

impl From<SedesError> for Error {
    fn from(error: SedesError) -> Self {
        Error::SedesError(error)
    }
}

use bincode::Error as SedesError;
use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Error as IOError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, HorsError>;

#[derive(Debug)]
/// Error exposed by hors.
pub struct HorsError {
    repr: Repr,
}

impl Error for HorsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.repr {
            Repr::Network(network_err) => network_err.source(),
            Repr::Parse(_) => None,
            Repr::IOError(io_err) => io_err.source(),
            Repr::SedesError(sedes_err) => sedes_err.source(),
        }
    }
}

impl Display for HorsError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.repr)
    }
}

#[derive(Debug)]
enum Repr {
    Network(reqwest::Error),
    IOError(IOError),
    SedesError(SedesError),
    Parse(&'static str),
}

impl HorsError {
    pub fn from_parse(reason: &'static str) -> HorsError {
        HorsError {
            repr: Repr::Parse(reason),
        }
    }
}

impl From<reqwest::Error> for HorsError {
    fn from(error: reqwest::Error) -> Self {
        HorsError {
            repr: Repr::Network(error),
        }
    }
}

impl From<IOError> for HorsError {
    fn from(error: IOError) -> Self {
        HorsError {
            repr: Repr::IOError(error),
        }
    }
}

impl From<SedesError> for HorsError {
    fn from(error: SedesError) -> Self {
        HorsError {
            repr: Repr::SedesError(error),
        }
    }
}

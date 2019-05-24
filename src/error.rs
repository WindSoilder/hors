use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Error as IOError;
use bincode::Error as SedesError;
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
            Repr::Network(network_err) => {
                return network_err.source();
            }
            Repr::Parse(_) => {
                return None;
            }
            Repr::IOError(io_err) => {
                return io_err.source();
            }
            Repr::SedesError(sedes_err) => {
                return sedes_err.source();
            }
        }
    }
}

impl Display for HorsError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Hor error occured, more information: {:?}", self.repr)
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
        return HorsError {
            repr: Repr::Parse(reason),
        };
    }
}

impl From<reqwest::Error> for HorsError {
    fn from(error: reqwest::Error) -> Self {
        return HorsError {
            repr: Repr::Network(error),
        };
    }
}

impl From<IOError> for HorsError {
    fn from(error: IOError) -> Self {
        return HorsError {
            repr: Repr::IOError(error),
        };
    }
}

impl From<SedesError> for HorsError {
    fn from(error: SedesError) -> Self {
        return HorsError {
            repr: Repr::SedesError(error),
        };
    }
}

use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, HorsError>;

#[derive(Debug)]
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

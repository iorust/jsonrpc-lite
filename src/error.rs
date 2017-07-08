use std::error;
use std::result;
use std::fmt;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidVersionParsed(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            InvalidVersionParsed(_) => "Serde Parsed something other than 2.0 as the JsonRpc version",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            InvalidVersionParsed(ref v) => write!(f, "Serde Parsed {} as the version instead of 2.0", v),
        }
    }
}

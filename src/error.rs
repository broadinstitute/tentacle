use std::fmt::{Display, Formatter, Debug};
use std::num::ParseIntError;

#[derive(Debug)]
pub(crate) enum Error {
    Tentacle(TentacleError),
    Reqwest(reqwest::Error),
    ParseInt(ParseIntError),
    SerdeJson(serde_json::Error),
    Io(std::io::Error)
}

#[derive(Debug)]
pub(crate) struct TentacleError {
    message: String,
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Tentacle(TentacleError::from(message))
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Error::Reqwest(reqwest_error)
    }
}

impl From<ParseIntError> for Error {
    fn from(parse_int_error: ParseIntError) -> Self {
        Error::ParseInt(parse_int_error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(serde_json_error: serde_json::Error) -> Self {
        Error::SerdeJson(serde_json_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::Io(io_error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Tentacle(tentacle_error) => {
                Display::fmt(tentacle_error, f)
            }
            Error::Reqwest(reqwest_error) => {
                Display::fmt(reqwest_error, f)
            }
            Error::ParseInt(parse_int_error) => {
                Display::fmt(parse_int_error, f)
            }
            Error::SerdeJson(serde_json_error) => {
                Display::fmt(serde_json_error, f)
            }
            Error::Io(io_error) => {
                Display::fmt(io_error, f)
            }
        }
    }
}

impl From<String> for TentacleError {
    fn from(message: String) -> Self {
        TentacleError { message }
    }
}

impl Display for TentacleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}


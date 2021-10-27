use std::fmt::{Display, Formatter, Debug};

#[derive(Debug)]
pub(crate) enum Error {
    Tentacle(TentacleError),
    Reqwest(reqwest::Error),
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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Tentacle(tentacle_error) => { Display::fmt(tentacle_error, f) }
            Error::Reqwest(reqwest_error) => { Display::fmt(reqwest_error, f) }
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


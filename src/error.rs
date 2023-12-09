use std::{error::Error, fmt::Display, io};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LumenError {
    NetworkError,
}

impl Error for LumenError {}

impl Display for LumenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

impl From<io::Error> for LumenError {
    fn from(error: io::Error) -> Self {
        Self::NetworkError
    }
}

impl From<serde_json::Error> for LumenError {
    fn from(error: serde_json::Error) -> Self {
        Self::NetworkError
    }
}

impl From<reqwest::Error> for LumenError {
    fn from(error: reqwest::Error) -> Self {
        Self::NetworkError
    }
}

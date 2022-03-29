use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    IO(std::io::Error),
    Custom(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::IO(err) => err.fmt(f),
            ServerError::Custom(message) => message.fmt(f),
        }
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> ServerError {
        ServerError::IO(err)
    }
}

impl std::error::Error for ServerError {}

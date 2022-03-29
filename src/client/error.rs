use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    IO(std::io::Error),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::IO(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for ClientError {
    fn from(err: std::io::Error) -> ClientError {
        ClientError::IO(err)
    }
}

impl std::error::Error for ClientError {}

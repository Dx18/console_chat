use std::fmt;

use console_chat::client::error::ClientError;
use console_chat::server::error::ServerError;

#[derive(Debug)]
pub enum Error {
    Server(ServerError),
    Client(ClientError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Server(err) => write!(f, "Server error: {}", err),
            Error::Client(err) => write!(f, "Client error: {}", err),
        }
    }
}

impl From<ServerError> for Error {
    fn from(err: ServerError) -> Error {
        Error::Server(err)
    }
}

impl From<ClientError> for Error {
    fn from(err: ClientError) -> Error {
        Error::Client(err)
    }
}

impl std::error::Error for Error {}

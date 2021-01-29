use actix_web::client;

#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    SendRequestError(client::SendRequestError),
    PayloadError(client::PayloadError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::SerdeError(error)
    }
}

impl From<client::SendRequestError> for Error {
    fn from(error: client::SendRequestError) -> Error {
        Error::SendRequestError(error)
    }
}

impl From<client::PayloadError> for Error {
    fn from(error: client::PayloadError) -> Error {
        Error::PayloadError(error)
    }
}

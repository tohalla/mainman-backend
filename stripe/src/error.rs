use actix_web::client;

#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    SendRequestError(client::SendRequestError),
    PayloadError(client::PayloadError),
    GenericError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        error!("Serde: {:?}", error);
        Error::SerdeError(error)
    }
}

impl From<client::SendRequestError> for Error {
    fn from(error: client::SendRequestError) -> Error {
        error!("SendRequestError: {:?}", error);
        Error::SendRequestError(error)
    }
}

impl From<client::PayloadError> for Error {
    fn from(error: client::PayloadError) -> Error {
        error!("PayloadError: {:?}", error);
        Error::PayloadError(error)
    }
}

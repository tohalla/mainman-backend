use actix_web::client;

#[derive(Debug)]
pub enum Error {
    SerdeJSONError(serde_json::Error),
    SerdeQSError(serde_qs::Error),
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
        Error::SerdeJSONError(error)
    }
}

impl From<serde_qs::Error> for Error {
    fn from(error: serde_qs::Error) -> Error {
        error!("Serde: {:?}", error);
        Error::SerdeQSError(error)
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

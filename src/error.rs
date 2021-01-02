use actix_http::ResponseBuilder;
use actix_web::{
    error::{BlockingError, ResponseError},
    http::{header, StatusCode},
    HttpResponse,
};
use bcrypt::BcryptError;
use diesel::{r2d2::PoolError, result::Error as DieselError};
use jsonwebtoken;
use std::io::{self};

#[derive(Debug)]
pub enum Error {
    NotFoundError,
    UnauthorizedError,
    BadRequestError,
    IOError(io::Error),
    EncodingError(String),
    InternalServerError(Option<String>),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::NotFoundError => StatusCode::NOT_FOUND,
            Error::UnauthorizedError => StatusCode::UNAUTHORIZED,
            Error::BadRequestError => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Error {
        match error {
            DieselError::DatabaseError(_, info) => Error::InternalServerError(
                info.details().map(|details| details.to_owned()),
            ),
            DieselError::NotFound => Error::NotFoundError,
            _ => Error::InternalServerError(None),
        }
    }
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Error {
        Error::InternalServerError(Some(format!("{:}", error)))
    }
}

impl From<BlockingError<Error>> for Error {
    fn from(error: BlockingError<Error>) -> Error {
        match error {
            BlockingError::Error(error) => error,
            BlockingError::Canceled => Error::InternalServerError(None),
        }
    }
}

impl From<BcryptError> for Error {
    fn from(error: BcryptError) -> Error {
        Error::InternalServerError(Some(format!("{:}", error)))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Error {
        Error::EncodingError(format!("{:}", error))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Error {
        Error::EncodingError(format!("{:}", error))
    }
}

use actix_http::{ResponseBuilder, ResponseError};
use actix_web::{
    error::BlockingError,
    http::{header, StatusCode},
    HttpResponse,
};
use bcrypt::BcryptError;
use diesel::{r2d2::PoolError, result::Error as DieselError};
use futures::channel::mpsc::SendError;
use jsonwebtoken;

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    status: u16,
    title: Option<String>,
    detail: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    errors: Vec<Error>,
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(json!(self))
    }

    fn status_code(&self) -> StatusCode {
        if let Some(error) = self.errors.first() {
            if let Ok(status) = StatusCode::from_u16(error.status) {
                return status;
            }
        }
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ErrorResponse::from(self.to_owned()).fmt(f)
    }
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        ErrorResponse {
            errors: vec![error],
        }
    }
}

impl Error {
    pub fn unauthorized() -> Self {
        Error {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            ..Self::default()
        }
    }
}

impl Default for Error {
    fn default() -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            title: None,
            detail: None,
        }
    }
}

impl From<diesel::result::Error> for ErrorResponse {
    fn from(error: diesel::result::Error) -> ErrorResponse {
        ErrorResponse::from(match error {
            DieselError::DatabaseError(_, info) => Error {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                title: Some("Database error".to_owned()),
                detail: info.details().map(|details| details.to_owned()),
            },
            DieselError::NotFound => Error {
                status: StatusCode::NOT_FOUND.as_u16(),
                title: Some("Database error".to_owned()),
                detail: Some("Not found".to_owned()),
            },
            _ => Error::default(),
        })
    }
}

impl From<PoolError> for ErrorResponse {
    fn from(error: PoolError) -> ErrorResponse {
        ErrorResponse::from(Error {
            status: StatusCode::NOT_FOUND.as_u16(),
            title: Some("Pool error".to_owned()),
            detail: Some(error.to_string()),
        })
    }
}

impl From<SendError> for ErrorResponse {
    fn from(_: SendError) -> ErrorResponse {
        ErrorResponse::from(Error::default())
    }
}

impl From<BlockingError<Error>> for Error {
    fn from(error: BlockingError<Error>) -> Error {
        match error {
            BlockingError::Error(error) => error,
            BlockingError::Canceled => Error::default(),
        }
    }
}

impl From<BcryptError> for ErrorResponse {
    fn from(_: BcryptError) -> ErrorResponse {
        ErrorResponse::from(Error::default())
    }
}

impl From<std::str::Utf8Error> for ErrorResponse {
    fn from(error: std::str::Utf8Error) -> ErrorResponse {
        ErrorResponse::from(Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            title: Some("Encoding error".to_owned()),
            detail: Some(error.to_string()),
        })
    }
}

impl From<jsonwebtoken::errors::Error> for ErrorResponse {
    fn from(error: jsonwebtoken::errors::Error) -> ErrorResponse {
        ErrorResponse::from(Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            title: Some("JWT error".to_owned()),
            detail: Some(error.to_string()),
        })
    }
}

impl From<stripe::error::Error> for ErrorResponse {
    fn from(error: stripe::error::Error) -> ErrorResponse {
        ErrorResponse::from(Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            title: Some("stripe error".to_owned()),
            detail: Some(error.to_string()),
        })
    }
}

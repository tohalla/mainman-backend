use actix_http::{ResponseBuilder, ResponseError};
use actix_web::{
    error::{BlockingError, JsonPayloadError},
    http::{header, StatusCode},
    HttpResponse,
};
use bcrypt::BcryptError;
use diesel::{r2d2::PoolError, result::Error as DieselError};
use futures::channel::mpsc::SendError;
use jsonwebtoken;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    errors: Vec<Error>,
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, mime::APPLICATION_JSON)
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

impl ErrorResponse {
    pub fn new() -> Self {
        ErrorResponse { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: Error) -> &mut Self {
        self.errors.push(error);
        self
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

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status.as_u16();
        self
    }

    pub fn detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.to_owned());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn source(mut self, source: &str) -> Self {
        self.source = Some(source.to_owned());
        self
    }
}

impl Default for Error {
    fn default() -> Self {
        Error {
            source: None,
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
                source: None,
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                title: Some("Database error".to_owned()),
                detail: info.details().map(|details| details.to_owned()),
            },
            DieselError::NotFound => Error {
                source: None,
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
            source: None,
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
            source: None,
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            title: Some("Encoding error".to_owned()),
            detail: Some(error.to_string()),
        })
    }
}

impl From<jsonwebtoken::errors::Error> for ErrorResponse {
    fn from(error: jsonwebtoken::errors::Error) -> ErrorResponse {
        ErrorResponse::from(Error {
            source: None,
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            title: Some("JWT error".to_owned()),
            detail: Some(error.to_string()),
        })
    }
}

impl From<stripe::error::Error> for ErrorResponse {
    fn from(error: stripe::error::Error) -> ErrorResponse {
        ErrorResponse::from(Error {
            source: None,
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            title: Some("Stripe error".to_owned()),
            detail: Some(error.to_string()),
        })
    }
}

impl From<JsonPayloadError> for ErrorResponse {
    fn from(error: JsonPayloadError) -> ErrorResponse {
        ErrorResponse::from(Error {
            source: None,
            status: StatusCode::BAD_REQUEST.as_u16(),
            title: Some("Invalid payload".to_owned()),
            detail: Some(error.to_string()),
        })
    }
}

impl From<ValidationErrors> for ErrorResponse {
    fn from(validation_errors: ValidationErrors) -> ErrorResponse {
        let mut errors = Vec::<Error>::new();
        for (field, error) in validation_errors.into_errors().iter() {
            match error {
                ValidationErrorsKind::Field(validation_errors) => {
                    for error in validation_errors {
                        errors.push(Error::from_validation_error(field, error))
                    }
                }
                _ => {}
            };
        }

        ErrorResponse { errors }
    }
}

impl From<StatusCode> for ErrorResponse {
    fn from(status: StatusCode) -> Self {
        ErrorResponse::from(Error {
            source: None,
            status: status.as_u16(),
            title: None,
            detail: None,
        })
    }
}

impl Error {
    fn from_validation_error(field: &str, error: &ValidationError) -> Self {
        Error {
            source: Some(field.to_owned()),
            title: Some("Validation error".to_owned()),
            detail: error.message.clone().map(|message| message.into_owned()),
            status: StatusCode::BAD_REQUEST.as_u16(),
        }
    }
}

use actix_http::ResponseBuilder;
use actix_web::{
    error::{BlockingError, ResponseError},
    http::{header, StatusCode},
    HttpResponse,
};
use bcrypt::BcryptError;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use failure::Fail;

#[derive(Fail, Debug, PartialEq)]
pub enum ApiError {
    #[fail(display = "Internal server error")]
    InternalServerError,
    #[fail(display = "Bad request {}", _0)]
    BadRequest(String),
    #[fail(display = "")]
    BlockingError,
    #[fail(display = "")]
    NotFound,
    #[fail(display = "")]
    PoolError,
    #[fail(display = "")]
    Unauthorized,
    #[fail(display = "")]
    EncodingError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DBError> for ApiError {
    fn from(error: DBError) -> ApiError {
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info
                        .details()
                        .unwrap_or_else(|| info.message())
                        .to_string();
                    return ApiError::BadRequest(message);
                }
                ApiError::InternalServerError
            }
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<PoolError> for ApiError {
    fn from(_: PoolError) -> ApiError {
        ApiError::PoolError
    }
}

impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> ApiError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ApiError::BlockingError,
        }
    }
}

impl From<BcryptError> for ApiError {
    fn from(_: BcryptError) -> ApiError {
        ApiError::BlockingError
    }
}

impl From<std::str::Utf8Error> for ApiError {
    fn from(_: std::str::Utf8Error) -> ApiError {
        ApiError::EncodingError
    }
}

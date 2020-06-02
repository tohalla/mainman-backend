use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use bcrypt::BcryptError;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use failure::Fail;

#[derive(Fail, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    #[fail(display = "Internal server error")]
    InternalServerError(String),
    #[fail(display = "Bad request {}", _0)]
    BadRequest(String),
    #[fail(display = "")]
    BlockingError(String),
    #[fail(display = "")]
    CacheError(String),
    #[fail(display = "")]
    NotFound(String),
    #[fail(display = "")]
    PoolError(String),
    #[fail(display = "")]
    ValidationError(Vec<String>),
    #[fail(display = "")]
    Unauthorized(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            ApiError::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity()
                    .json::<ErrorResponse>(errors.to_vec().into())
            }
            ApiError::Unauthorized(error) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
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
                ApiError::InternalServerError("Unknown database error".into())
            }
            _ => ApiError::InternalServerError("Unknown database error".into()),
        }
    }
}

impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        ApiError::PoolError(error.to_string())
    }
}

impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> ApiError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => {
                ApiError::BlockingError("Thread blocking error".into())
            }
        }
    }
}

impl From<BcryptError> for ApiError {
    fn from(error: BcryptError) -> ApiError {
        ApiError::BlockingError(error.to_string())
    }
}

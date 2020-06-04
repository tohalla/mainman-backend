use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};

use crate::db::Pool;
use crate::error::ApiError;

pub mod extractor;
mod handler;
pub mod middleware;
pub mod routes;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub account_id: i32,
    pub exp: i64,
}

pub fn encode_jwt(claim: Claim) -> Result<String, ApiError> {
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(std::env::var("JWT_KEY").unwrap().as_ref()),
    )
    .map_err(|_| ApiError::InternalServerError)
}

#[allow(dead_code)]
pub fn decode_jwt(token: &str) -> Result<Claim, ApiError> {
    decode::<Claim>(
        token,
        &DecodingKey::from_secret(std::env::var("JWT_KEY").unwrap().as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ApiError::Unauthorized)
}

pub fn find_by_auth_details(
    pool: &Pool,
    payload: handler::AuthPayload,
) -> Result<i32, ApiError> {
    use crate::schema::account::dsl::*;

    let conn = pool.get()?;
    let result = account
        .select((id, password))
        .filter(email.eq(payload.email))
        .first::<(i32, Vec<u8>)>(&conn)
        .map_err(|_| ApiError::NotFound)?;

    if verify(payload.password, std::str::from_utf8(&result.1)?)? {
        Ok(result.0)
    } else {
        Err(ApiError::Unauthorized)
    }
}

impl From<i32> for Claim {
    fn from(account_id: i32) -> Self {
        Claim {
            account_id,
            exp: (Utc::now() + Duration::hours(1)).timestamp(),
        }
    }
}

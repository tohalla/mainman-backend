use actix_identity::RequestIdentity;
use actix_web::{dev, FromRequest, HttpRequest, Result};
use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel::{dsl::sql, prelude::*, sql_types};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};

use crate::db::Pool;
use crate::error::ApiError;

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
        .filter(
            sql("lower(email) = ")
                .bind::<sql_types::Text, _>(payload.email.to_lowercase()),
        )
        .first::<(i32, Vec<u8>)>(&conn)
        .map_err(|_| ApiError::Unauthorized)?;

    if verify(payload.password, std::str::from_utf8(&result.1)?)? {
        Ok(result.0)
    } else {
        Err(ApiError::Unauthorized)
    }
}

#[derive(Debug)]
pub struct AuthenticationDetails {
    pub account_id: i32,
}

impl AuthenticationDetails {
    fn from_identity(identity: Option<String>) -> Result<Self, ApiError> {
        if let Some(auth_token) = identity {
            match decode_jwt(&auth_token) {
                Ok(claim) => {
                    return Ok(AuthenticationDetails {
                        account_id: claim.account_id,
                    })
                }
                Err(_) => return Err(ApiError::Unauthorized),
            };
        }
        Err(ApiError::Unauthorized)
    }
}

impl FromRequest for AuthenticationDetails {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        match AuthenticationDetails::from_identity(req.get_identity()) {
            Ok(authentication_details) => return ok(authentication_details),
            Err(e) => return err(e),
        }
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

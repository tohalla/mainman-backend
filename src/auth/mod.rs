use actix_web::{
    cookie::{Cookie, CookieBuilder, SameSite},
    dev, FromRequest, HttpMessage, HttpRequest, Result,
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel::{dsl::sql, prelude::*, sql_query, sql_types};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use uuid::Uuid;

use crate::db::Pool;
use crate::error::ApiError;
use crate::schema::refresh_token;

mod handler;
pub mod middleware;
pub mod routes;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub account_id: i32,
    pub exp: i64,
}

#[derive(Debug, QueryableByName)]
#[table_name = "refresh_token"]
pub struct AuthenticationDetails {
    pub account_id: i32,
}

#[derive(Debug, QueryableByName)]
#[table_name = "refresh_token"]
pub struct RefreshTokenIdentifier(#[column_name = "token"] pub Uuid);

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

pub fn create_authentication_tokens(
    pool: &Pool,
    account_id: Option<i32>,
) -> Result<(Cookie, Cookie), ApiError> {
    let authentication_token = match account_id {
        Some(account_id) => encode_jwt(account_id.into())?,
        None => "".to_string(),
    };

    Ok((
        decorate_cookie(Cookie::build(
            "authorization",
            authentication_token.clone(),
        ))
        .finish(),
        create_refresh_token(&pool, account_id, authentication_token.clone())?,
    ))
}

fn decorate_cookie(cookie: CookieBuilder) -> CookieBuilder {
    cookie.same_site(SameSite::Strict).http_only(true).path("/")
}

fn create_refresh_token(
    pool: &Pool,
    account_id: Option<i32>,
    authentication_token: String,
) -> Result<Cookie, ApiError> {
    let conn = pool.get()?;

    let account_id = match account_id {
        Some(account_id) => sql_query(
            "SELECT generate_refresh_token($1::INTEGER, $2::TEXT) token",
        )
        .bind::<sql_types::Integer, _>(account_id)
        .bind::<sql_types::Text, _>(&authentication_token)
        .get_result::<RefreshTokenIdentifier>(&conn)?
        .0
        .to_string(),
        None => "".to_string(),
    };

    Ok(decorate_cookie(Cookie::build("refresh-token", account_id)).finish())
}

pub fn validate_refresh_token(
    pool: &Pool,
    token: &Uuid,
    authentication_token: Option<String>,
) -> Result<AuthenticationDetails, ApiError> {
    if let Some(authentication_token) = authentication_token {
        let conn = pool.get()?;
        let result = sql_query(
            "SELECT validate_refresh_token($1::UUID, $2::TEXT) account_id",
        )
        .bind::<sql_types::Uuid, _>(token)
        .bind::<sql_types::Text, _>(authentication_token)
        .get_result::<AuthenticationDetails>(&conn)?;

        return Ok(result);
    }
    Err(ApiError::Unauthorized)
}

impl AuthenticationDetails {
    fn from_identity(identity: Option<String>) -> Result<Self, ApiError> {
        if let Some(authentication_token) = identity {
            match decode_jwt(&authentication_token) {
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
        match AuthenticationDetails::from_identity(
            req.cookie("authorization")
                .map(|cookie| cookie.value().to_string()),
        ) {
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

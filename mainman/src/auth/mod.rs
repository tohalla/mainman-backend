use actix_web::{
    cookie::{Cookie, CookieBuilder, SameSite},
    dev, FromRequest, HttpMessage, HttpRequest, HttpResponse, Result,
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel::{dsl::sql, prelude::*, sql_query, sql_types};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use uuid::Uuid;

use crate::{
    config,
    db::Connection,
    error::{Error, ErrorResponse},
    schema::refresh_token,
    MainmanResult,
};

mod handler;
pub mod middleware;
pub mod routes;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct AuthCookies<'a> {
    pub auth: Cookie<'a>,
    pub refresh: Cookie<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub account_id: i64,
    pub exp: i64,
}

#[derive(Debug, QueryableByName)]
#[table_name = "refresh_token"]
pub struct RefreshToken(#[column_name = "token"] pub Uuid);

impl Claim {
    pub fn jwt(&self) -> MainmanResult<String> {
        Ok(encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(
                std::env::var("JWT_KEY").unwrap_or("".to_owned()).as_ref(),
            ),
        )?)
    }

    pub fn decode(token: &str, validation: &Validation) -> MainmanResult<Self> {
        Ok(decode(
            token,
            &DecodingKey::from_secret(
                std::env::var("JWT_KEY").unwrap().as_ref(),
            ),
            validation,
        )
        .map(|data| data.claims)?)
    }

    fn from_identity(
        identity: Option<String>,
        validation: &Validation,
    ) -> MainmanResult<Self> {
        match identity
            .and_then(|identity| AuthCookies::parse_auth_token(&identity))
        {
            Some(authentication_token) => {
                Claim::decode(&authentication_token, validation)
            }
            None => Err(Error::unauthorized().into()),
        }
    }
}

impl Credentials {
    pub fn claim(&self, conn: &Connection) -> MainmanResult<Claim> {
        use crate::schema::account::dsl::*;

        let result = account
            .select((id, password))
            .filter(
                sql("lower(email) = ")
                    .bind::<sql_types::Text, _>(self.email.to_lowercase()),
            )
            .first::<(i64, Vec<u8>)>(conn)
            .map_err(|_| Error::unauthorized())?;

        if verify(&self.password, std::str::from_utf8(&result.1)?)? {
            Ok(Claim::from(result.0))
        } else {
            Err(Error::unauthorized().into())
        }
    }
}

impl<'a> AuthCookies<'a> {
    pub fn cookies(claim: &Claim, conn: &Connection) -> MainmanResult<Self> {
        let authentication_token = claim.jwt()?;

        Ok(AuthCookies {
            auth: Self::auth_cookie(&authentication_token),
            refresh: AuthCookies::refresh_cookie(
                claim,
                &authentication_token,
                conn,
            )?,
        })
    }

    pub fn clear() -> HttpResponse {
        HttpResponse::Ok()
            // TODO: drop refresh-token from the database
            .cookie(Self::decorate(Cookie::build("refresh-token", "")).finish())
            .cookie(Self::decorate(Cookie::build("authorization", "")).finish())
            .finish()
    }

    pub fn parse_auth_token(value: &str) -> Option<String> {
        if value.starts_with(config::AUTH_TOKEN_PREFIX) {
            Some(value[config::AUTH_TOKEN_PREFIX.len()..].to_owned())
        } else {
            None
        }
    }

    pub fn auth_cookie(authentication_token: &String) -> Cookie<'a> {
        Self::decorate(Cookie::build(
            "authorization",
            format!("{}{}", config::AUTH_TOKEN_PREFIX, authentication_token),
        ))
        .finish()
    }

    fn refresh_cookie(
        claim: &Claim,
        authentication_token: &String,
        conn: &Connection,
    ) -> MainmanResult<Cookie<'a>> {
        let token = sql_query(
            "SELECT generate_refresh_token($1::INTEGER, $2::TEXT) token",
        )
        .bind::<sql_types::BigInt, _>(claim.account_id)
        .bind::<sql_types::Text, _>(&authentication_token)
        .get_result::<RefreshToken>(conn)?
        .0
        .to_string();

        Ok(Self::decorate(Cookie::build("refresh-token", token)).finish())
    }

    fn decorate(builder: CookieBuilder) -> CookieBuilder {
        builder
            .same_site(SameSite::Strict)
            .http_only(true)
            .path("/")
    }
}

impl<'a> Into<HttpResponse> for AuthCookies<'a> {
    fn into(self) -> HttpResponse {
        HttpResponse::Accepted()
            .cookie(self.auth)
            .cookie(self.refresh)
            .finish()
    }
}

impl RefreshToken {
    pub fn validate_refresh_token(
        &self,
        authentication_token: Option<String>,
        conn: &Connection,
    ) -> MainmanResult<Claim> {
        if let Some(authentication_token) = authentication_token {
            let account_id =
                sql::<sql_types::BigInt>("SELECT validate_refresh_token(")
                    .bind::<sql_types::Uuid, _>(self.0)
                    .sql(",")
                    .bind::<sql_types::Text, _>(authentication_token)
                    .sql(")")
                    .get_result::<i64>(conn)?;

            return Ok(account_id.into());
        }
        Err(Error::unauthorized().into())
    }
}

impl FromRequest for Claim {
    type Error = ErrorResponse;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let mut validation = Validation::default();
        validation.validate_exp = false;
        match Claim::from_identity(
            req.cookie("authorization")
                .map(|cookie| cookie.value().to_string()),
            &validation,
        ) {
            Ok(claim) => ok(claim),
            Err(e) => err(e.into()),
        }
    }
}

impl From<i64> for Claim {
    fn from(account_id: i64) -> Self {
        Claim {
            account_id,
            exp: (Utc::now() + Duration::hours(1)).timestamp(),
        }
    }
}

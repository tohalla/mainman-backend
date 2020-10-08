use actix_web::{
    cookie::Cookie,
    web::{block, Data, Json},
    HttpMessage, HttpRequest, HttpResponse,
};
use uuid::Uuid;

use crate::account;
use crate::db::Pool;
use crate::error::ApiError;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[post("")]
pub async fn authenticate(
    pool: Data<Pool>,
    payload: Json<AuthPayload>,
) -> Result<HttpResponse, ApiError> {
    let account_id = super::find_by_auth_details(&pool, payload.into_inner())?;

    let (authentication_token, refresh_token) =
        super::create_authentication_tokens(&pool, Some(account_id))?;

    Ok(HttpResponse::Ok()
        .cookie(authentication_token)
        .cookie(refresh_token)
        .finish())
}

#[post("/refresh")]
pub async fn refresh_session(
    pool: Data<Pool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let authentication_token = req
        .cookie("authorization")
        .map(|authentication_token| authentication_token.value().to_string());
    let authentication_details = req
        .cookie("refresh-token")
        .and_then(|refresh_token| Uuid::parse_str(refresh_token.value()).ok())
        .and_then(|refresh_token| {
            super::validate_refresh_token(
                &pool,
                &refresh_token,
                authentication_token,
            )
            .ok()
        });

    if authentication_details.is_none() {
        return Err(ApiError::Unauthorized);
    }

    let (authentication_token, refresh_token) =
        super::create_authentication_tokens(
            &pool,
            authentication_details.map(|authentication_details| {
                authentication_details.account_id
            }),
        )?;

    Ok(HttpResponse::Ok()
        .cookie(authentication_token)
        .cookie(refresh_token)
        .finish())
}

#[get("")]
pub async fn get_account(
    pool: Data<Pool>,
    authentication_details: super::AuthenticationDetails,
) -> Result<Json<account::handler::AccountResponse>, ApiError> {
    let account =
        block(move || account::find(&pool, authentication_details.account_id))
            .await?;
    Ok(Json(account))
}

#[delete("")]
pub async fn sign_out() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok()
        .cookie(
            super::decorate_cookie(Cookie::build("refresh-token", "")).finish(),
        )
        .cookie(
            super::decorate_cookie(Cookie::build("authorization", "")).finish(),
        )
        .finish())
}

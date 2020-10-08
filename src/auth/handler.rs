use actix_web::{
    cookie::Cookie,
    web::{block, Data, Json},
    HttpResponse,
};

use crate::account;
use crate::db::Pool;
use crate::error::ApiError;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

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

pub async fn get_account(
    pool: Data<Pool>,
    authentication_details: super::AuthenticationDetails,
) -> Result<Json<account::handler::AccountResponse>, ApiError> {
    let account =
        block(move || account::find(&pool, authentication_details.account_id))
            .await?;
    Ok(Json(account))
}

pub async fn sign_out() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok()
        .cookie(Cookie::build("", "").finish())
        .finish())
}

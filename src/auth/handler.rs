use actix_identity::Identity;
use actix_web::{
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
    identity: Identity,
    pool: Data<Pool>,
    payload: Json<AuthPayload>,
) -> Result<HttpResponse, ApiError> {
    let account_id =
        block(move || super::find_by_auth_details(&pool, payload.into_inner()))
            .await?;

    identity.remember(super::encode_jwt(account_id.into())?);

    Ok(HttpResponse::Ok().finish())
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

pub async fn sign_out(identity: Identity) -> Result<HttpResponse, ApiError> {
    identity.forget();
    Ok(HttpResponse::Ok().finish())
}

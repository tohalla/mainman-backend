use actix_identity::Identity;
use actix_web::{
    web::{block, Data, Json},
    HttpResponse, Result,
};

use crate::db::Pool;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

pub async fn authenticate(
    identity: Identity,
    pool: Data<Pool>,
    payload: Json<AuthPayload>,
) -> Result<HttpResponse> {
    let account_id =
        block(move || super::find_by_auth_details(&pool, payload.into_inner()))
            .await?;

    identity.remember(super::encode_jwt(account_id.into())?);

    Ok(HttpResponse::Found().finish())
}

pub async fn sign_out(identity: Identity) -> Result<HttpResponse> {
    identity.forget();
    Ok(HttpResponse::Ok().finish())
}

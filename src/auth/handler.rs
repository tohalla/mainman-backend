use actix_web::{
    web::{Data, Json},
    HttpMessage, HttpRequest, HttpResponse,
};
use uuid::Uuid;

use crate::{
    account::Account, db::Pool, error::Error, MainmanResponse, MainmanResult,
};

#[post("")]
pub async fn authenticate(
    pool: Data<Pool>,
    payload: Json<super::Credentials>,
) -> MainmanResult<HttpResponse> {
    let conn = pool.get()?;
    let claim = payload.into_inner().claim(&conn)?;

    Ok(super::AuthCookies::cookies(&claim, &conn)?.into())
}

#[post("/refresh")]
pub async fn refresh_session(
    pool: Data<Pool>,
    req: HttpRequest,
) -> MainmanResult<HttpResponse> {
    let conn = pool.get()?;
    let authentication_token = req.cookie("authorization").and_then(|cookie| {
        super::AuthCookies::parse_auth_token(cookie.value())
    });
    let claim = req
        .cookie("refresh-token")
        .and_then(|refresh_token| Uuid::parse_str(refresh_token.value()).ok())
        .and_then(|refresh_token| {
            super::RefreshToken(refresh_token)
                .validate_refresh_token(authentication_token, &conn)
                .ok()
        });

    match claim {
        Some(claim) => Ok(super::AuthCookies::cookies(&claim, &conn)?.into()),
        None => Err(Error::UnauthorizedError),
    }
}

#[get("")]
pub async fn get_account(
    pool: Data<Pool>,
    claim: super::Claim,
) -> MainmanResponse<Account> {
    Ok(Account::get(claim.account_id, &pool.get()?)?.into())
}

#[delete("")]
pub async fn sign_out() -> HttpResponse {
    super::AuthCookies::clear()
}

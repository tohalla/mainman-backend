use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::{account::Account, db::Pool, MainmanResponse, MainmanResult};

#[post("")]
pub async fn authenticate(
    pool: Data<Pool>,
    payload: Json<super::Credentials>,
) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    let claim = payload.into_inner().claim(conn)?;

    Ok(super::AuthCookies::cookies(&claim, conn)?.into())
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

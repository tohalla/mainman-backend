use actix_web::web::{Data, HttpResponse, Json, Path};
use bcrypt::{hash, DEFAULT_COST};
use heck::TitleCase;
use validator::Validate;

use super::*;
use crate::{
    auth::Claim,
    db::Pool,
    organisation::{
        invite::{NewOrganisationInvite, OrganisationInvite},
        Organisation, OrganisationAccount,
    },
    MainmanResponse,
};

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct NewAccountPayload {
    pub first_name: String,
    pub last_name: String,
    #[validate(email(message = "invalidEmail"))]
    pub email: String,
    #[validate(length(min = 5))]
    pub password: String,
}

#[post("")]
pub async fn create_account(
    pool: Data<Pool>,
    payload: Json<NewAccountPayload>,
) -> MainmanResponse<Account> {
    payload.validate()?;
    Ok(NewAccount {
        email: &payload.email,
        first_name: &payload.first_name.to_title_case(),
        last_name: &payload.last_name.to_title_case(),
        password: hash(&payload.password, DEFAULT_COST)?.as_bytes(),
    }
    .create(&pool.get()?)?
    .into())
}

#[get("")]
pub async fn account(
    pool: Data<Pool>,
    account_id: Path<i32>,
) -> MainmanResponse<Account> {
    Ok(Account::get(*account_id, &pool.get()?)?.into())
}

#[get("invites")]
pub async fn invites(
    pool: Data<Pool>,
    account_id: Path<i32>,
) -> MainmanResponse<Vec<OrganisationInvite>> {
    let conn = &pool.get()?;
    Ok(Account::get(*account_id, conn)?.invites(conn)?.into())
}

// organisation routes

#[get("")]
pub async fn organisation_accounts(
    pool: Data<Pool>,
    organisation_id: Path<i32>,
) -> MainmanResponse<Vec<PublicAccount>> {
    let conn = &pool.get()?;
    Ok(Organisation::get(*organisation_id, conn)?
        .accounts(conn)?
        .into())
}

#[get("invites")]
pub async fn organisation_invites(
    pool: Data<Pool>,
    organisation_id: Path<i32>,
) -> MainmanResponse<Vec<OrganisationInvite>> {
    let conn = &pool.get()?;
    Ok(Organisation::get(*organisation_id, conn)?
        .invites(conn)?
        .into())
}

#[post("invites")]
pub async fn invite_account(
    pool: Data<Pool>,
    payload: Json<NewOrganisationInvite>,
    organisation_id: Path<i32>,
) -> MainmanResponse<OrganisationInvite> {
    payload.validate()?;
    Ok(NewOrganisationInvite {
        organisation: *organisation_id,
        email: payload.email.to_lowercase(),
        ..payload.into_inner()
    }
    .create(&pool.get()?)?
    .into())
}

#[post("invites/{uuid}")]
pub async fn accept_invite(
    pool: Data<Pool>,
    path: Path<(i32, uuid::Uuid)>,
    claim: Claim,
) -> MainmanResponse<OrganisationAccount> {
    let conn = &pool.get()?;
    Ok(OrganisationInvite::get((*path).0, (*path).1, conn)?
        .accept(&claim, conn)?
        .into())
}

#[delete("invites/{uuid}")]
pub async fn delete_invite(
    pool: Data<Pool>,
    path: Path<(i32, uuid::Uuid)>,
) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    OrganisationInvite::get((*path).0, (*path).1, conn)?.delete(conn)?;
    Ok(HttpResponse::Accepted().finish())
}

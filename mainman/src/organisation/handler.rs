use actix_web::web::{Data, Json, Path};

use super::*;
use crate::{auth::Claim, db::Pool, MainmanResponse};

#[get("")]
pub async fn get_organisations(
    pool: Data<Pool>,
    claim: Claim,
) -> MainmanResponse<Vec<Organisation>> {
    Ok(Organisation::all(claim.account_id, &pool.get()?)?.into())
}

#[post("")]
pub async fn create_organisation(
    pool: Data<Pool>,
    payload: Json<NewOrganisation>,
    claim: Claim,
) -> MainmanResponse<Organisation> {
    Ok(NewOrganisation {
        admin_account: claim.account_id,
        ..payload.into_inner()
    }
    .create(&pool.get()?)?
    .into())
}

// /{organisation_id}

#[get("")]
pub async fn get_organisation(
    pool: Data<Pool>,
    organisation_id: Path<i32>,
) -> MainmanResponse<Organisation> {
    Ok(Organisation::get(*organisation_id, &pool.get()?)?.into())
}

#[patch("")]
pub async fn patch_organisation<'a>(
    pool: Data<Pool>,
    payload: Json<PatchOrganisation>,
    organisation_id: Path<i32>,
) -> MainmanResponse<Organisation> {
    let conn = &pool.get()?;
    Ok(Organisation::get(*organisation_id, &conn)?
        .patch(&payload, &conn)?
        .into())
}

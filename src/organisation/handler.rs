use actix_web::web::{Data, Json, Path};

use super::*;
use crate::{db::Pool, MainmanResponse};

pub async fn get_organisation(
    pool: Data<Pool>,
    organisation_id: Path<i32>,
) -> MainmanResponse<Organisation> {
    Ok(Organisation::get(*organisation_id, &pool.get()?)?.into())
}

pub async fn get_organisations(
    pool: Data<Pool>,
    authentication_details: crate::auth::AuthenticationDetails,
) -> MainmanResponse<Vec<Organisation>> {
    Ok(
        Organisation::all(authentication_details.account_id, &pool.get()?)?
            .into(),
    )
}

pub async fn create_organisation(
    pool: Data<Pool>,
    payload: Json<NewOrganisation>,
    authentication_details: crate::auth::AuthenticationDetails,
) -> MainmanResponse<Organisation> {
    Ok(NewOrganisation {
        admin_account: authentication_details.account_id,
        ..payload.into_inner()
    }
    .create(&pool.get()?)?
    .into())
}

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

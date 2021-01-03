use actix_web::web::{Data, Json, Path};

use super::*;
use crate::{db::Pool, MainmanResult};

pub async fn get_organisation(
    pool: Data<Pool>,
    organisation_id: Path<i32>,
) -> MainmanResult<Json<Organisation>> {
    Ok(Json(Organisation::get(*organisation_id, &pool.get()?)?))
}

pub async fn get_organisations(
    pool: Data<Pool>,
    authentication_details: crate::auth::AuthenticationDetails,
) -> MainmanResult<Json<Vec<Organisation>>> {
    Ok(Json(Organisation::all(
        authentication_details.account_id,
        &pool.get()?,
    )?))
}

pub async fn create_organisation(
    pool: Data<Pool>,
    payload: Json<NewOrganisation>,
    authentication_details: crate::auth::AuthenticationDetails,
) -> MainmanResult<Json<Organisation>> {
    Ok(Json(
        NewOrganisation {
            admin_account: authentication_details.account_id,
            ..payload.into_inner()
        }
        .create(&pool.get()?)?,
    ))
}

pub async fn patch_organisation<'a>(
    pool: Data<Pool>,
    payload: Json<PatchOrganisation>,
    organisation_id: Path<i32>,
) -> MainmanResult<Json<Organisation>> {
    let conn = &pool.get()?;
    Ok(Json(
        Organisation::get(*organisation_id, &conn)?.patch(&payload, &conn)?,
    ))
}

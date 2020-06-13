use actix_web::web::{block, Data, Json, Path};

use super::*;
use crate::db::Pool;
use crate::error::ApiError;

pub async fn get_organisation(
    pool: Data<Pool>,
    id: Path<i32>,
) -> Result<Json<Organisation>, ApiError> {
    let organisation = block(move || find(&pool, *id)).await?;
    Ok(Json(organisation))
}

pub async fn get_organisations(
    pool: Data<Pool>,
    authentication_details: crate::auth::AuthenticationDetails,
) -> Result<Json<Vec<Organisation>>, ApiError> {
    let organisation =
        block(move || get_all(&pool, authentication_details.account_id))
            .await?;
    Ok(Json(organisation))
}

pub async fn create_organisation(
    pool: Data<Pool>,
    payload: Json<CreateOrganisation>,
    authentication_details: crate::auth::AuthenticationDetails,
) -> Result<Json<Organisation>, ApiError> {
    let mut insertable = payload.into_inner();
    insertable.admin_account = authentication_details.account_id;

    let organisation = block(move || create(&pool, insertable)).await?;
    Ok(Json(organisation))
}

pub async fn patch_organisation(
    pool: Data<Pool>,
    payload: Json<PatchOrganisation>,
) -> Result<Json<Organisation>, ApiError> {
    let organisation =
        block(move || patch(&pool, payload.into_inner())).await?;
    Ok(Json(organisation))
}

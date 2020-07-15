use actix_web::web::{block, Data, Json, Path};

use super::*;
use crate::db::Pool;
use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct CreateOrganisationPayload {
    name: String,
    organisation_identifier: String,
    locale: String,
}

pub async fn get_organisation(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> Result<Json<Organisation>, ApiError> {
    let organisation = block(move || find(&pool, *organisation)).await?;
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
    payload: Json<CreateOrganisationPayload>,
    authentication_details: crate::auth::AuthenticationDetails,
) -> Result<Json<Organisation>, ApiError> {
    let organisation = block(move || {
        create(
            &pool,
            CreateOrganisation {
                admin_account: authentication_details.account_id,
                name: &payload.name,
                organisation_identifier: Some(&payload.organisation_identifier),
                locale: &payload.locale,
            },
        )
    })
    .await?;
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

use actix_web::web::{block, Data, Json, Path};

use super::*;
use crate::db::Pool;
use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct CreateOrganisationPayload {
    name: String,
    organisation_identifier: Option<String>,
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
                organisation_identifier: payload
                    .organisation_identifier
                    .as_deref()
                    .unwrap_or(""),
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
    organisation: Path<i32>,
) -> Result<Json<Organisation>, ApiError> {
    let organisation_res = block(move || {
        patch(
            &pool,
            &PatchOrganisation {
                id: *organisation,
                ..payload.into_inner()
            },
        )
    })
    .await?;
    Ok(Json(organisation_res))
}

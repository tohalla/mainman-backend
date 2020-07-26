use actix_web::web::{block, Data, Json, Path};
use uuid::Uuid;

use super::*;
use crate::db::Pool;
use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct CreateAppliancePayload {
    name: String,
    description: Option<String>,
}

pub async fn get_appliance(
    pool: Data<Pool>,
    hash: Path<Uuid>,
) -> Result<Json<Appliance>, ApiError> {
    let appliance = block(move || find(&pool, *hash)).await?;
    Ok(Json(appliance))
}

pub async fn get_appliances(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> Result<Json<Vec<Appliance>>, ApiError> {
    let appliance = block(move || get_all(&pool, *organisation)).await?;
    Ok(Json(appliance))
}

pub async fn create_appliance(
    pool: Data<Pool>,
    payload: Json<CreateAppliancePayload>,
    organisation: Path<i32>,
) -> Result<Json<Appliance>, ApiError> {
    let appliance = block(move || {
        create(
            &pool,
            &CreateAppliance {
                organisation: *organisation,
                name: &payload.name,
                description: &payload.description.as_deref().unwrap_or(""),
            },
        )
    })
    .await?;
    Ok(Json(appliance))
}

pub async fn patch_appliance(
    pool: Data<Pool>,
    payload: Json<PatchAppliance>,
    appliance: Path<Uuid>,
) -> Result<Json<Appliance>, ApiError> {
    let appliance_res = block(move || {
        patch(
            &pool,
            &PatchAppliance {
                hash: *appliance,
                ..payload.into_inner()
            },
        )
    })
    .await?;
    Ok(Json(appliance_res))
}

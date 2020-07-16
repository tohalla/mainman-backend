use actix_web::web::{block, Data, Json, Path};
use uuid::Uuid;

use super::*;
use crate::db::Pool;
use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct CreateAppliancePayload {
    name: String,
    description: String,
}

pub async fn get_appliance(
    pool: Data<Pool>,
    path: Path<(i32, Uuid)>,
) -> Result<Json<Appliance>, ApiError> {
    let appliance = block(move || find(&pool, path.1)).await?;
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
            CreateAppliance {
                name: &payload.name,
                description: Some(&payload.description),
                organisation: *organisation,
            },
        )
    })
    .await?;
    Ok(Json(appliance))
}

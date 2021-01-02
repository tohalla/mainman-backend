use actix_web::web::{block, Data, Json, Path};
use uuid::Uuid;

use super::*;
use crate::{db::Pool, MainmanResult};

#[derive(Debug, Deserialize)]
pub struct CreateAppliancePayload {
    name: String,
    description: Option<String>,
}

#[get("{hash}")]
pub async fn get_appliance(
    pool: Data<Pool>,
    hash: Path<Uuid>,
) -> MainmanResult<Json<Appliance>> {
    let appliance = block(move || find(&pool, *hash)).await?;
    Ok(Json(appliance))
}

pub async fn get_appliances(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Vec<Appliance>>> {
    let appliance = block(move || get_all(&pool, *organisation)).await?;
    Ok(Json(appliance))
}

pub async fn create_appliance(
    pool: Data<Pool>,
    payload: Json<CreateAppliancePayload>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Appliance>> {
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

#[patch("{hash}")]
pub async fn patch_appliance(
    pool: Data<Pool>,
    payload: Json<PatchAppliance>,
    hash: Path<Uuid>,
) -> MainmanResult<Json<Appliance>> {
    let appliance_res = block(move || {
        patch(
            &pool,
            &PatchAppliance {
                hash: *hash,
                ..payload.into_inner()
            },
        )
    })
    .await?;
    Ok(Json(appliance_res))
}

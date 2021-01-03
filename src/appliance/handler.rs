use actix_web::web::{Data, Json, Path};
use uuid::Uuid;

use super::*;
use crate::{db::Pool, MainmanResult};

#[get("{hash}")]
pub async fn get_appliance(
    pool: Data<Pool>,
    hash: Path<Uuid>,
) -> MainmanResult<Json<Appliance>> {
    Ok(Json(Appliance::get(*hash, &pool.get()?)?))
}

#[get("")]
pub async fn get_appliances(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Vec<Appliance>>> {
    Ok(Json(Appliance::by_organisation(
        *organisation,
        &pool.get()?,
    )?))
}

#[post("")]
pub async fn create_appliance(
    pool: Data<Pool>,
    payload: Json<NewAppliance>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Appliance>> {
    Ok(Json(
        NewAppliance {
            organisation: *organisation,
            ..payload.into_inner()
        }
        .insert(&pool.get()?)?,
    ))
}

#[patch("{hash}")]
pub async fn patch_appliance(
    pool: Data<Pool>,
    payload: Json<PatchAppliance>,
    hash: Path<Uuid>,
) -> MainmanResult<Json<Appliance>> {
    let conn = &pool.get()?;

    Ok(Json(Appliance::get(*hash, &conn)?.patch(&payload, &conn)?))
}

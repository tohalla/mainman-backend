use actix_web::web::{Data, Json, Path};
use uuid::Uuid;

use super::*;
use crate::{db::Pool, MainmanResult};

#[get("{hash}")]
pub async fn get_entity(
    pool: Data<Pool>,
    hash: Path<Uuid>,
) -> MainmanResult<Json<Entity>> {
    Ok(Json(Entity::get(*hash, &pool.get()?)?))
}

#[get("")]
pub async fn get_entities(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Vec<Entity>>> {
    Ok(Json(Entity::by_organisation(*organisation, &pool.get()?)?))
}

#[post("")]
pub async fn create_entity(
    pool: Data<Pool>,
    payload: Json<NewEntity>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Entity>> {
    Ok(Json(
        NewEntity {
            organisation: *organisation,
            ..payload.into_inner()
        }
        .insert(&pool.get()?)?,
    ))
}

#[patch("{hash}")]
pub async fn patch_entity(
    pool: Data<Pool>,
    payload: Json<PatchEntity>,
    hash: Path<Uuid>,
) -> MainmanResult<Json<Entity>> {
    let conn = &pool.get()?;

    Ok(Json(Entity::get(*hash, &conn)?.patch(&payload, &conn)?))
}

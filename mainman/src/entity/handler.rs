use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use uuid::Uuid;

use super::*;
use crate::{
    db::Pool,
    maintainer::{Maintainer, MaintainerEntity},
    MainmanResponse, MainmanResult,
};

#[get("{hash}")]
pub async fn get_entity(
    pool: Data<Pool>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Entity> {
    Ok(Entity::get((*path).1, (*path).0, &pool.get()?)?.into())
}

#[get("")]
pub async fn get_entities(
    pool: Data<Pool>,
    organisation_id: Path<i32>,
) -> MainmanResponse<Vec<Entity>> {
    let conn = &pool.get()?;
    Ok(Organisation::get(*organisation_id, conn)?
        .entities(conn)?
        .into())
}

#[post("")]
pub async fn create_entity(
    pool: Data<Pool>,
    payload: Json<NewEntity>,
    organisation: Path<i32>,
) -> MainmanResponse<Entity> {
    Ok(NewEntity {
        organisation: *organisation,
        ..payload.into_inner()
    }
    .create(&pool.get()?)?
    .into())
}

#[patch("{hash}")]
pub async fn patch_entity(
    pool: Data<Pool>,
    payload: Json<PatchEntity>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Entity> {
    let conn = &pool.get()?;
    Ok(Entity::get((*path).1, (*path).0, &conn)?
        .patch(&payload, &conn)?
        .into())
}

#[get("{hash}/maintainers")]
pub async fn maintainers(
    pool: Data<Pool>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Vec<Maintainer>> {
    let conn = &pool.get()?;
    Ok(Entity::get((*path).1, (*path).0, &conn)?
        .maintainers(&conn)?
        .into())
}

#[post("{hash}/maintainers")]
pub async fn add_maintainers(
    pool: Data<Pool>,
    payload: Json<Vec<i32>>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Vec<MaintainerEntity>> {
    let conn = &pool.get()?;
    // sepparate fetch for checking access to entity
    let entity = Entity::get((*path).1, (*path).0, &conn)?;
    Ok(payload
        .iter()
        .map(|maintainer| MaintainerEntity {
            organisation: (*path).0,
            entity: entity.hash,
            maintainer: *maintainer,
        })
        .collect::<Vec<_>>()
        .create(&conn)?
        .into())
}

#[delete("{hash}/maintainers")]
pub async fn delete_maintainers(
    pool: Data<Pool>,
    payload: Json<Vec<i32>>,
    path: Path<(i32, Uuid)>,
) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    Entity::get((*path).1, (*path).0, &conn)?
        .delete_maintainers(&*payload, &conn)?;
    Ok(HttpResponse::Ok().finish())
}

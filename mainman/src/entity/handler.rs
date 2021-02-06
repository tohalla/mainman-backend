use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use uuid::Uuid;

use super::*;
use crate::{
    db::Pool,
    maintainer::{Maintainer, MaintainerEntity},
    maintenance::{
        maintenance_request::MaintenanceRequest,
        maintenance_trigger::{MaintenanceTrigger, NewMaintenanceTrigger},
    },
    MainmanResponse, MainmanResult,
};

#[get("{uuid}")]
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

#[patch("{uuid}")]
pub async fn patch_entity(
    pool: Data<Pool>,
    payload: Json<PatchEntity>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Entity> {
    let conn = &pool.get()?;
    Ok(Entity::get((*path).1, (*path).0, conn)?
        .patch(&payload, conn)?
        .into())
}

#[post("{uuid}/maintainers")]
pub async fn add_maintainers(
    pool: Data<Pool>,
    payload: Json<Vec<i32>>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Vec<MaintainerEntity>> {
    let conn = &pool.get()?;
    // sepparate fetch for checking access to entity
    let entity = Entity::get((*path).1, (*path).0, conn)?;
    Ok(payload
        .iter()
        .map(|maintainer| MaintainerEntity {
            organisation: (*path).0,
            entity: entity.uuid,
            maintainer: *maintainer,
        })
        .collect::<Vec<_>>()
        .create(conn)?
        .into())
}

#[delete("{uuid}/maintainers")]
pub async fn delete_maintainers(
    pool: Data<Pool>,
    payload: Json<Vec<i32>>,
    path: Path<(i32, Uuid)>,
) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    Entity::get((*path).1, (*path).0, conn)?
        .delete_maintainers(&*payload, conn)?;
    Ok(HttpResponse::Accepted().finish())
}

#[get("{uuid}/maintainers")]
pub async fn maintainers(
    pool: Data<Pool>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Vec<Maintainer>> {
    let conn = &pool.get()?;
    Ok(Entity::get((*path).1, (*path).0, conn)?
        .maintainers(conn)?
        .into())
}

#[get("{uuid}/maintenance-requests")]
pub async fn maintenance_requests(
    pool: Data<Pool>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Vec<MaintenanceRequest>> {
    let conn = &pool.get()?;
    Ok(Entity::get((*path).1, (*path).0, conn)?
        .maintenance_requests(conn)?
        .into())
}

#[get("{uuid}/maintenance-triggers")]
pub async fn maintenance_triggers(
    pool: Data<Pool>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Vec<MaintenanceTrigger>> {
    let conn = &pool.get()?;
    Ok(Entity::get((*path).1, (*path).0, conn)?
        .maintenance_triggers(conn)?
        .into())
}

#[post("{uuid}/maintenance-triggers")]
pub async fn create_maintenance_trigger(
    pool: Data<Pool>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<MaintenanceTrigger> {
    let conn = &pool.get()?;
    // sepparate fetch for checking access to entity
    Entity::get((*path).1, (*path).0, conn)?;
    Ok(NewMaintenanceTrigger { entity: (*path).1 }
        .create(conn)?
        .into())
}

#[delete("{uuid}/maintenance-triggers/{trigger_uuid}")]
pub async fn delete_maintenance_trigger(
    pool: Data<Pool>,
    path: Path<(i32, Uuid, Uuid)>,
) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    MaintenanceTrigger::delete((*path).1, (*path).2, conn)?;
    Ok(HttpResponse::Accepted().finish())
}

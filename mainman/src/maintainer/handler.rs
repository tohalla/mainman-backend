use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};

use super::*;
use crate::{db::Pool, entity::Entity, MainmanResponse, MainmanResult};

#[get("{maintainer_id}")]
pub async fn get_maintainer(
    pool: Data<Pool>,
    path: Path<(i64, i64)>,
) -> MainmanResponse<Maintainer> {
    Ok(Maintainer::get((*path).1, (*path).0, &pool.get()?)?.into())
}

#[get("")]
pub async fn get_maintainers(
    pool: Data<Pool>,
    organisation_id: Path<i64>,
) -> MainmanResponse<Vec<Maintainer>> {
    let conn = &pool.get()?;
    Ok(Organisation::get(*organisation_id, conn)?
        .maintainers(conn)?
        .into())
}

#[post("")]
pub async fn create_maintainer(
    pool: Data<Pool>,
    payload: Json<NewMaintainer>,
    organisation_id: Path<i64>,
) -> MainmanResponse<Maintainer> {
    Ok(NewMaintainer {
        organisation: *organisation_id,
        ..payload.into_inner()
    }
    .create(&pool.get()?)?
    .into())
}

#[patch("{maintainer_id}")]
pub async fn patch_maintainer(
    pool: Data<Pool>,
    payload: Json<PatchMaintainer>,
    path: Path<(i64, i64)>,
) -> MainmanResponse<Maintainer> {
    let conn = &pool.get()?;
    Ok(Maintainer::get((*path).1, (*path).0, conn)?
        .patch(&payload, conn)?
        .into())
}

#[get("{maintainer_id}/entities")]
pub async fn entities(pool: Data<Pool>, path: Path<(i64, i64)>) -> MainmanResponse<Vec<Entity>> {
    let conn = &pool.get()?;
    Ok(Maintainer::get((*path).1, (*path).0, conn)?
        .entities(conn)?
        .into())
}

#[post("{maintainer_id}/entities")]
pub async fn add_entities(
    pool: Data<Pool>,
    payload: Json<Vec<Uuid>>,
    path: Path<(i64, i64)>,
) -> MainmanResponse<Vec<MaintainerEntity>> {
    let conn = &pool.get()?;
    // sepparate fetch for checking access to maintainer
    let maintainer = Maintainer::get((*path).1, (*path).0, conn)?;
    Ok(payload
        .iter()
        .map(|entity| MaintainerEntity {
            organisation: (*path).0,
            maintainer: maintainer.id,
            entity: *entity,
        })
        .collect::<Vec<_>>()
        .create(conn)?
        .into())
}

#[delete("{maintainer_id}/entities")]
pub async fn delete_entities(
    pool: Data<Pool>,
    payload: Json<Vec<Uuid>>,
    path: Path<(i64, i64)>,
) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    Maintainer::get((*path).1, (*path).0, conn)?.delete_entities(&*payload, conn)?;
    Ok(HttpResponse::Ok().finish())
}

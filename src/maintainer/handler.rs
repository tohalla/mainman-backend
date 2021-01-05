use actix_web::web::{Data, Json, Path};

use super::*;
use crate::{db::Pool, entity::Entity, MainmanResponse};

#[get("{maintainer_id}")]
pub async fn get_maintainer(
    pool: Data<Pool>,
    path: Path<(i32, i32)>,
) -> MainmanResponse<Maintainer> {
    Ok(Maintainer::get(path.1, &pool.get()?)?.into())
}

#[get("")]
pub async fn get_maintainers(
    pool: Data<Pool>,
    organisation_id: Path<i32>,
) -> MainmanResponse<Vec<Maintainer>> {
    Ok(Maintainer::by_organisation(*organisation_id, &pool.get()?)?.into())
}

#[post("")]
pub async fn create_maintainer(
    pool: Data<Pool>,
    payload: Json<NewMaintainer>,
    organisation_id: Path<i32>,
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
    path: Path<(i32, i32)>,
) -> MainmanResponse<Maintainer> {
    let conn = &pool.get()?;
    Ok(Maintainer::get(path.1, &conn)?
        .patch(&payload, &conn)?
        .into())
}

#[get("{maintainer_id}/entities")]
pub async fn entities(
    pool: Data<Pool>,
    path: Path<(i32, i32)>,
) -> MainmanResponse<Vec<Entity>> {
    Ok(Entity::by_maintainer(path.1, &pool.get()?)?.into())
}

#[post("{maintainer_id}/entities")]
pub async fn add_entities(
    pool: Data<Pool>,
    payload: Json<Vec<Uuid>>,
    path: Path<(i32, i32)>,
) -> MainmanResponse<Vec<MaintainerEntity>> {
    Ok(payload
        .iter()
        .map(|entity| MaintainerEntity {
            organisation: (*path).0,
            maintainer: (*path).1,
            entity: *entity,
        })
        .collect::<Vec<_>>()
        .create(&pool.get()?)?
        .into())
}

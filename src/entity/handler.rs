use actix_web::web::{Data, Json, Path};
use uuid::Uuid;

use super::*;
use crate::{db::Pool, maintainer::MaintainerEntity, MainmanResponse};

#[get("{hash}")]
pub async fn get_entity(
    pool: Data<Pool>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Entity> {
    Ok(Entity::get(path.1, &pool.get()?)?.into())
}

#[get("")]
pub async fn get_entities(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> MainmanResponse<Vec<Entity>> {
    Ok(Entity::by_organisation(*organisation, &pool.get()?)?.into())
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
    Ok(Entity::get(path.1, &conn)?.patch(&payload, &conn)?.into())
}

#[post("{hash}/maintainers")]
pub async fn add_maintainers(
    pool: Data<Pool>,
    payload: Json<Vec<i32>>,
    path: Path<(i32, Uuid)>,
) -> MainmanResponse<Vec<MaintainerEntity>> {
    Ok(payload
        .iter()
        .map(|maintainer| MaintainerEntity {
            organisation: (*path).0,
            entity: (*path).1,
            maintainer: *maintainer,
        })
        .collect::<Vec<_>>()
        .create(&pool.get()?)?
        .into())
}

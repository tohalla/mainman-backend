use actix_web::web::{Data, Json, Path};

use super::*;
use crate::{db::Pool, MainmanResponse};

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
    .insert(&pool.get()?)?
    .into())
}

#[patch("{hash}")]
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

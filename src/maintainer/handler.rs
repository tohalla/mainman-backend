use actix_web::web::{Data, Json, Path};

use super::*;
use crate::{db::Pool, MainmanResult};

#[get("{maintainer_id}")]
pub async fn get_maintainer(
    pool: Data<Pool>,
    maintainer_id: Path<i32>,
) -> MainmanResult<Json<Maintainer>> {
    Ok(Json(Maintainer::get(*maintainer_id, &pool.get()?)?))
}

#[get("")]
pub async fn get_maintainers(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Vec<Maintainer>>> {
    Ok(Json(Maintainer::by_organisation(
        *organisation,
        &pool.get()?,
    )?))
}

#[post("")]
pub async fn create_maintainer(
    pool: Data<Pool>,
    payload: Json<NewMaintainer>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Maintainer>> {
    Ok(Json(
        NewMaintainer {
            organisation: *organisation,
            ..payload.into_inner()
        }
        .insert(&pool.get()?)?,
    ))
}

#[patch("{hash}")]
pub async fn patch_maintainer(
    pool: Data<Pool>,
    payload: Json<PatchMaintainer>,
    maintainer_id: Path<i32>,
) -> MainmanResult<Json<Maintainer>> {
    let conn = &pool.get()?;

    Ok(Json(
        Maintainer::get(*maintainer_id, &conn)?.patch(&payload, &conn)?,
    ))
}

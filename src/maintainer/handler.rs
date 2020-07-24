use actix_web::web::{block, Data, Json, Path};

use super::*;
use crate::db::Pool;
use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct CreateMaintainerPayload {
    name: String,
    description: Option<String>,
}

pub async fn get_maintainer(
    pool: Data<Pool>,
    path: Path<(i32, i32)>,
) -> Result<Json<Maintainer>, ApiError> {
    let maintainer = block(move || find(&pool, path.1)).await?;
    Ok(Json(maintainer))
}

pub async fn get_maintainers(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> Result<Json<Vec<Maintainer>>, ApiError> {
    let maintainer = block(move || get_all(&pool, *organisation)).await?;
    Ok(Json(maintainer))
}

pub async fn create_maintainer(
    pool: Data<Pool>,
    payload: Json<CreateMaintainer>,
    organisation: Path<i32>,
) -> Result<Json<Maintainer>, ApiError> {
    let maintainer = block(move || {
        create(
            &pool,
            CreateMaintainer {
                organisation: *organisation,
                ..payload.into_inner()
            },
        )
    })
    .await?;
    Ok(Json(maintainer))
}

pub async fn patch_maintainer(
    pool: Data<Pool>,
    payload: Json<PatchMaintainer>,
    maintainer: Path<i32>,
) -> Result<Json<Maintainer>, ApiError> {
    let maintainer_res = block(move || {
        patch(
            &pool,
            &PatchMaintainer {
                id: *maintainer,
                ..payload.into_inner()
            },
        )
    })
    .await?;
    Ok(Json(maintainer_res))
}

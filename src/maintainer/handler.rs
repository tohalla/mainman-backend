use actix_web::web::{block, Data, Json, Path};

use super::*;
use crate::db::Pool;
use crate::error::ApiError;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct CreateMaintainerPayload {
    details: Option<serde_json::Value>,
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
    payload: Json<CreateMaintainerPayload>,
    organisation: Path<i32>,
) -> Result<Json<Maintainer>, ApiError> {
    let maintainer = block(move || {
        create(
            &pool,
            &CreateMaintainer {
                organisation: *organisation,
                account: None,
                details: payload.details.clone().unwrap_or(json!({})),
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

use actix_web::web::{block, Data, Json, Path};
use serde_json;

use super::*;
use crate::{db::Pool, MainmanResult};

#[derive(Debug, Deserialize)]
pub struct CreateMaintainerPayload {
    details: Option<serde_json::Value>,
}

#[get("/{maintainer_id}")]
pub async fn get_maintainer(
    pool: Data<Pool>,
    maintainer_id: Path<i32>,
) -> MainmanResult<Json<Maintainer>> {
    let maintainer = block(move || find(&pool, *maintainer_id)).await?;
    Ok(Json(maintainer))
}

pub async fn get_maintainers(
    pool: Data<Pool>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Vec<Maintainer>>> {
    let maintainer = block(move || get_all(&pool, *organisation)).await?;
    Ok(Json(maintainer))
}

pub async fn create_maintainer(
    pool: Data<Pool>,
    payload: Json<CreateMaintainerPayload>,
    organisation: Path<i32>,
) -> MainmanResult<Json<Maintainer>> {
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

#[get("/{maintainer_id}")]
pub async fn patch_maintainer(
    pool: Data<Pool>,
    payload: Json<PatchMaintainer>,
    maintainer: Path<i32>,
) -> MainmanResult<Json<Maintainer>> {
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

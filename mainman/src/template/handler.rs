use actix_web::web::{Data, Json, Path};

use super::*;
use crate::{db::Pool, MainmanResponse};

#[get("")]
pub async fn get_templates(
    pool: Data<Pool>,
    organisation_id: Path<i64>,
) -> MainmanResponse<Vec<Template>> {
    let conn = &pool.get()?;
    Ok(Organisation::get(*organisation_id, conn)?
        .templates(conn)?
        .into())
}

#[get("{id}")]
pub async fn get_template(pool: Data<Pool>, path: Path<(i64, i64)>) -> MainmanResponse<Template> {
    Ok(Template::get((*path).1, (*path).0, &pool.get()?)?.into())
}

#[post("")]
pub async fn create_template(
    pool: Data<Pool>,
    payload: Json<NewTemplate>,
    organisation: Path<i64>,
) -> MainmanResponse<Template> {
    Ok(NewTemplate {
        organisation: *organisation,
        ..payload.into_inner()
    }
    .create(&pool.get()?)?
    .into())
}

#[patch("{id}")]
pub async fn patch_template(
    pool: Data<Pool>,
    payload: Json<PatchTemplate>,
    path: Path<(i64, i64)>,
) -> MainmanResponse<Template> {
    let conn = &pool.get()?;
    Ok(Template::get((*path).1, (*path).0, conn)?
        .patch(&payload, conn)?
        .into())
}

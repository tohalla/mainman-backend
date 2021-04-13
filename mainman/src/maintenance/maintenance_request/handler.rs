use actix_web::web::{Data, Path};
use uuid::Uuid;

use super::*;
use crate::{db::Pool, entity::Entity, MainmanResponse};

// entity routes -- /organisation/{organisation}/entities/{entity}/maintenance/requests/

#[get("")]
pub async fn maintenance_requests(
    pool: Data<Pool>,
    path: Path<(i64, Uuid)>,
) -> MainmanResponse<Vec<MaintenanceRequest>> {
    let conn = &pool.get()?;
    Ok(Entity::get((*path).1, (*path).0, conn)?
        .maintenance_requests(conn)?
        .into())
}

#[get("{id}")]
pub async fn maintenance_request(
    pool: Data<Pool>,
    path: Path<(i64, Uuid, i64)>,
    _: Entity,
) -> MainmanResponse<MaintenanceRequest> {
    Ok(MaintenanceRequest::get((*path).2, &pool.get()?)?.into())
}

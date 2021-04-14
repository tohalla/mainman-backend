use actix_web::web::{Data, Path};
use uuid::Uuid;

use super::*;
use crate::{db::Pool, MainmanResponse};

#[post("{uuid}/accept")]
pub async fn accept(
    pool: Data<Pool>,
    uuid: Path<Uuid>,
) -> MainmanResponse<MaintenanceTask> {
    let conn = &pool.get()?;
    Ok(MaintenanceTask::get(*uuid, conn)?.accept(conn)?.into())
}

#[post("{uuid}/resolve")]
pub async fn resolve(
    pool: Data<Pool>,
    uuid: Path<Uuid>,
) -> MainmanResponse<MaintenanceTask> {
    Ok(MaintenanceTask::get(*uuid, &pool.get()?)?.into())
}

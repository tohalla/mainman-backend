use actix_web::web::{Data, Json, Path};
use uuid::Uuid;

use super::*;
use crate::{
    db::Pool,
    maintenance::maintenance_request::{
        MaintenanceRequest, NewMaintenanceRequest,
    },
    MainmanResponse,
};

#[post("")]
pub async fn create_maintenance_request(
    pool: Data<Pool>,
    payload: Json<NewMaintenanceRequest>,
    uuid: Path<Uuid>,
) -> MainmanResponse<MaintenanceRequest> {
    let conn = &pool.get()?;
    let maintenance_trigger = MaintenanceTrigger::get(*uuid, conn)?;

    Ok(NewMaintenanceRequest {
        entity: maintenance_trigger.entity,
        maintenance_trigger: maintenance_trigger.uuid,
        ..payload.into_inner()
    }
    .create(conn)?
    .into())
}

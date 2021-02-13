use actix_web::web::{Data, Json, Path};
use std::sync::Mutex;
use uuid::Uuid;

use super::*;
use crate::{
    auth::Claim,
    db::Pool,
    events::{Broadcaster, Message},
    maintenance::maintenance_request::{
        MaintenanceRequest, NewMaintenanceRequest,
    },
    MainmanResponse,
};

#[post("{uuid}")]
pub async fn create_maintenance_request(
    broker: Data<Mutex<Broadcaster>>,
    pool: Data<Pool>,
    payload: Json<NewMaintenanceRequest>,
    uuid: Path<Uuid>,
    claim: Claim,
) -> MainmanResponse<MaintenanceRequest> {
    let conn = &pool.get()?;
    let maintenance_trigger = MaintenanceTrigger::get(*uuid, conn)?;

    let maintenance_request = NewMaintenanceRequest {
        entity: maintenance_trigger.entity,
        maintenance_trigger: maintenance_trigger.uuid,
        ..payload.into_inner()
    }
    .create(conn)?;

    if let Ok(mut broker) = broker.lock() {
        broker
            .send(Message {
                event: Some("maintenanceRequest"),
                data: &maintenance_request,
                recipient: claim.account_id,
            })
            .await?;
    }

    Ok(maintenance_request.into())
}

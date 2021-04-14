use actix_web::web::{Data, Json};
use std::sync::Mutex;

use super::*;
use crate::{
    auth::Claim,
    db::Pool,
    events::{Broadcaster, Message},
    MainmanResponse,
};

// entity routes -- /organisation/{organisation}/entities/{entity}/maintenance/events/

#[post("")]
pub async fn create_maintenance_event(
    broker: Data<Mutex<Broadcaster>>,
    claim: Claim,
    payload: Json<NewMaintenanceEvent>,
    pool: Data<Pool>,
    entity: Entity,
) -> MainmanResponse<MaintenanceEvent> {
    let conn = &pool.get()?;

    let maintenance_event = NewMaintenanceEvent {
        entity: entity.uuid,
        ..payload.into_inner()
    }
    .create(conn)?;

    if let Ok(mut broker) = broker.lock() {
        broker
            .send(Message {
                event: Some("maintenance_event"),
                data: &maintenance_event,
                recipient: claim.account_id,
            })
            .await?;
    }

    Ok(maintenance_event.into())
}

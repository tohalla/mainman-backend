use actix_web::web::{Data, Json};
use std::sync::Mutex;

use super::*;
use crate::{
    db::Pool,
    events::{Broadcaster, Message},
    organisation::Organisation,
    MainmanResponse,
};

// entity routes -- /organisation/{organisation}/entities/{entity}/maintenance/events/

#[post("")]
pub async fn create_maintenance_event(
    broker: Data<Mutex<Broadcaster>>,
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
            .send(
                &Message {
                    event: Some("maintenance_event"),
                    data: &maintenance_event,
                },
                &Organisation::get(entity.organisation, conn)?
                    .subscribers(conn)?,
            )
            .await?;
    }

    Ok(maintenance_event.into())
}

use actix_http::http::header::CONTENT_TYPE;
use actix_web::web::{Data, HttpResponse, Json, Path};
use std::sync::Mutex;
use uuid::Uuid;

use super::*;
use crate::{
    db::Pool,
    events::{Broadcaster, Message},
    maintenance::maintenance_request::{MaintenanceRequest, NewMaintenanceRequest},
    template::TEMPLATES,
    MainmanResponse,
};

#[get("{uuid}")]
pub async fn maintenance_trigger(
    pool: Data<Pool>,
    uuid: Path<Uuid>,
) -> MainmanResponse<MaintenanceTrigger> {
    Ok(MaintenanceTrigger::get(*uuid, &pool.get()?)?.into())
}

#[post("{uuid}")]
pub async fn create_maintenance_request(
    broker: Data<Mutex<Broadcaster>>,
    pool: Data<Pool>,
    payload: Json<NewMaintenanceRequest>,
    uuid: Path<Uuid>,
) -> MainmanResponse<MaintenanceRequest> {
    let conn = &pool.get()?;
    let trigger = MaintenanceTrigger::get(*uuid, conn)?;

    let maintenance_request = NewMaintenanceRequest {
        entity: trigger.entity,
        maintenance_trigger: trigger.uuid,
        ..payload.into_inner()
    }
    .create(conn)?;

    if let Ok(mut broker) = broker.lock() {
        broker
            .send(
                &Message {
                    event: Some("maintenance_request"),
                    data: &maintenance_request,
                },
                &trigger.organisation(conn)?.subscribers(conn)?,
            )
            .await?;
    }

    Ok(maintenance_request.into())
}

#[get("{uuid}/template")]
pub async fn template(pool: Data<Pool>, uuid: Path<Uuid>) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    let trigger = DetailedMaintenanceTrigger::get(*uuid, conn)?;

    let mut ctx = tera::Context::new();
    ctx.insert("entity", &trigger.entity);

    let template = match trigger.template(conn)? {
        Some(template) => tera::Tera::one_off(&template.content, &ctx, false)?,
        None => TEMPLATES.render("base/en/maintenance_request.html", &ctx)?,
    };

    Ok(HttpResponse::Ok()
        .set_header(CONTENT_TYPE, mime::TEXT_HTML_UTF_8)
        .body(template))
}

// entity routes -- /organisation/{organisation}/entities/{entity}/maintenance/triggers/

#[get("")]
pub async fn maintenance_triggers(
    pool: Data<Pool>,
    path: Path<(i64, Uuid)>,
) -> MainmanResponse<Vec<MaintenanceTrigger>> {
    let conn = &pool.get()?;
    Ok(Entity::get((*path).1, (*path).0, conn)?
        .maintenance_triggers(conn)?
        .into())
}

#[post("")]
pub async fn create_maintenance_trigger(
    pool: Data<Pool>,
    path: Path<(i64, Uuid)>,
) -> MainmanResponse<MaintenanceTrigger> {
    let conn = &pool.get()?;
    // sepparate fetch for checking access to entity
    Entity::get((*path).1, (*path).0, conn)?;
    Ok(NewMaintenanceTrigger { entity: (*path).1 }
        .create(conn)?
        .into())
}

#[delete("{uuid}")]
pub async fn delete_maintenance_trigger(
    pool: Data<Pool>,
    path: Path<(i64, Uuid, Uuid)>,
) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    MaintenanceTrigger::delete((*path).1, (*path).2, conn)?;
    Ok(HttpResponse::Accepted().finish())
}

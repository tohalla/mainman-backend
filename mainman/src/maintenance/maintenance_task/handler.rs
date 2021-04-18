use actix_http::http::header::CONTENT_TYPE;
use actix_web::web::{Data, HttpResponse, Path};
use std::sync::Mutex;
use uuid::Uuid;

use super::*;
use crate::{
    db::Pool,
    events::{Broadcaster, Message},
    template::TEMPLATES,
    MainmanResponse,
};

#[get("{uuid}")]
pub async fn maintenance_task(
    pool: Data<Pool>,
    uuid: Path<Uuid>,
) -> MainmanResponse<MaintenanceTask> {
    Ok(MaintenanceTask::get(*uuid, &pool.get()?)?.into())
}

#[post("{uuid}/accept")]
pub async fn accept(pool: Data<Pool>, uuid: Path<Uuid>) -> MainmanResponse<MaintenanceTask> {
    let conn = &pool.get()?;
    Ok(MaintenanceTask::get(*uuid, conn)?.accept(conn)?.into())
}

#[post("{uuid}/resolve")]
pub async fn resolve(
    pool: Data<Pool>,
    uuid: Path<Uuid>,
    broker: Data<Mutex<Broadcaster>>,
) -> MainmanResponse<MaintenanceTask> {
    let conn = &pool.get()?;
    let (task, event) = MaintenanceTask::get(*uuid, conn)?.resolve(conn)?;

    if let Ok(mut broker) = broker.lock() {
        broker
            .send(
                &Message {
                    event: Some("maintenance_event"),
                    data: &event,
                },
                &task.organisation(conn)?.subscribers(conn)?,
            )
            .await?;
    }

    Ok(task.into())
}

#[get("{uuid}/template")]
pub async fn template(pool: Data<Pool>, uuid: Path<Uuid>) -> MainmanResult<HttpResponse> {
    let conn = &pool.get()?;
    let task = DetailedMaintenanceTask::get(*uuid, conn)?;

    let mut ctx = tera::Context::new();
    ctx.insert("entity", &task.entity);
    ctx.insert("event", &task.maintenance_event);
    ctx.insert("task", &task.maintenance_task);

    let template = match task.template(conn)? {
        Some(template) => tera::Tera::one_off(&template.content, &ctx, false)?,
        None => TEMPLATES.render("base/en/maintenance_report.html", &ctx)?,
    };

    Ok(HttpResponse::Ok()
        .set_header(CONTENT_TYPE, mime::TEXT_HTML_UTF_8)
        .body(template))
}

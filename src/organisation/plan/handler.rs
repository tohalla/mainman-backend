use actix_web::web::Data;

use super::Plan;
use crate::{db::Pool, MainmanResponse};

#[get("")]
pub async fn get_plans(pool: Data<Pool>) -> MainmanResponse<Vec<Plan>> {
    Ok(Plan::all(&pool.get()?)?.into())
}

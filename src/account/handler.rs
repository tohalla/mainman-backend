use actix_web::web::{Data, Json, Path};
use bcrypt::{hash, DEFAULT_COST};
use heck::TitleCase;

use super::{Account, NewAccount};
use crate::{db::Pool, MainmanResult};

#[derive(Clone, Debug, Deserialize)]
pub struct NewAccountPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[post("")]
pub async fn create_account(
    pool: Data<Pool>,
    payload: Json<NewAccountPayload>,
) -> MainmanResult<Json<Account>> {
    Ok(Json(
        NewAccount {
            email: &payload.email,
            first_name: &payload.first_name.to_title_case(),
            last_name: &payload.last_name.to_title_case(),
            password: hash(&payload.password, DEFAULT_COST)?.as_bytes(),
        }
        .insert(&pool.get()?)?,
    ))
}

pub async fn get_account(
    pool: Data<Pool>,
    account_id: Path<i32>,
) -> MainmanResult<Json<Account>> {
    Ok(Json(Account::get(*account_id, &pool.get()?)?))
}

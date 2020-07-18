use actix_web::web::{block, Data, Json, Path};
use bcrypt::{hash, DEFAULT_COST};

use super::{create, find, Account, CreateAccount};
use crate::db::Pool;
use crate::error::ApiError;
use heck::TitleCase;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountResponse {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub retype_password: String,
}

pub async fn create_account(
    pool: Data<Pool>,
    payload: Json<CreateAccountPayload>,
) -> Result<Json<AccountResponse>, ApiError> {
    if payload.password != payload.retype_password {
        return Err(ApiError::ValidationError);
    }

    let account = block(move || {
        create(
            &pool,
            CreateAccount {
                email: &payload.email,
                first_name: &payload.first_name.to_title_case(),
                last_name: &payload.last_name.to_title_case(),
                password: hash(&payload.password, DEFAULT_COST)?.as_bytes(),
            },
        )
    })
    .await?;
    Ok(Json(account))
}

pub async fn get_account(
    pool: Data<Pool>,
    account_id: Path<i32>,
) -> Result<Json<AccountResponse>, ApiError> {
    let account = block(move || find(&pool, *account_id)).await?;
    Ok(Json(account))
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        AccountResponse {
            id: account.id,
            first_name: account.first_name,
            last_name: account.last_name,
            email: account.email,
        }
    }
}

use diesel::prelude::*;

use crate::{db::Connection, schema::plan, MainmanResult};

pub mod handler;

#[derive(Debug, Serialize, Queryable, Associations, Identifiable)]
#[table_name = "plan"]
pub struct Plan {
    pub id: i32,
    pub name: String,
    pub entities: Option<i32>,
    pub maintainers: Option<i32>,
    pub accounts: Option<i32>,
    #[serde(skip)]
    pub is_public: bool,
    #[serde(skip)]
    pub stripe_product: Option<String>,
    #[serde(skip)]
    pub stripe_price: Option<String>,
}

impl Plan {
    pub fn all(conn: &Connection) -> MainmanResult<Vec<Plan>> {
        Ok(plan::table.load::<Plan>(conn)?)
    }
}

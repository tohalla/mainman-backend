use diesel::prelude::*;

use crate::{db::Connection, schema::template_type, MainmanResult};

#[allow(dead_code)]
pub static MAINTENANCE_REQUEST: &str = "maintenance_request";
#[allow(dead_code)]
pub static MAINTENANCE_REPORT: &str = "maintenance_report";

#[derive(Debug, Associations, Serialize, Queryable, Identifiable)]
#[table_name = "template_type"]
pub struct TemplateType {
    pub id: i32,
    pub name: String,
}

impl TemplateType {
    pub fn all(&self, conn: &Connection) -> MainmanResult<Vec<Self>> {
        Ok(template_type::table.load::<Self>(conn)?)
    }
}

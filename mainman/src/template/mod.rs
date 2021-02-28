use chrono::NaiveDateTime;

use crate::{
    organisation::Organisation,
    schema::{template, template_type},
};

mod handler;
pub mod routes;

#[derive(Debug, Associations, Serialize, Queryable, Identifiable)]
#[belongs_to(Organisation, foreign_key = "organisation")]
#[belongs_to(TemplateType, foreign_key = "template_type")]
#[table_name = "template"]
pub struct Template {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub organisation: Option<i32>,
    pub name: Option<String>,
    pub content: serde_json::Value,
    pub is_draft: bool,
    pub template_type: i32,
}

#[derive(Debug, Associations, Serialize, Queryable, Identifiable)]
#[table_name = "template_type"]
pub struct TemplateType {
    pub id: i64,
    pub name: String,
}

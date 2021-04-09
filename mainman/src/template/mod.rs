use chrono::NaiveDateTime;
use diesel::prelude::*;

use self::template_type::TemplateType;
use crate::{
    db::{Connection, Creatable},
    organisation::Organisation,
    schema::template,
    MainmanResult,
};

mod handler;
pub mod routes;
pub mod template_type;

#[derive(Debug, Associations, Serialize, Queryable, Identifiable)]
#[belongs_to(Organisation, foreign_key = "organisation")]
#[belongs_to(TemplateType, foreign_key = "template_type")]
#[table_name = "template"]
pub struct Template {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub organisation: Option<i64>,
    pub name: Option<String>,
    pub content: String,
    pub is_draft: bool,
    pub template_type: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "template"]
pub struct NewTemplate {
    name: String,
    content: String,
    is_draft: bool,
    template_type: i32,
    #[serde(skip_deserializing)]
    organisation: i64,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "template"]
pub struct PatchTemplate {
    name: Option<String>,
    content: Option<String>,
    is_draft: Option<bool>,
    template_type: Option<i32>,
}

impl Template {
    pub fn get(
        id: i64,
        organisation: Option<i64>,
        conn: &Connection,
    ) -> MainmanResult<Self> {
        Ok(template::table
            .find(id)
            .filter(
                template::organisation
                    .eq(organisation)
                    .or(template::organisation.is_null()),
            )
            .first::<Self>(conn)?)
    }

    pub fn public(conn: &Connection) -> MainmanResult<Vec<Self>> {
        Ok(template::table
            .filter(template::organisation.is_null())
            .load::<Self>(conn)?)
    }

    pub fn patch(
        &self,
        payload: &PatchTemplate,
        conn: &Connection,
    ) -> MainmanResult<Self> {
        Ok(diesel::update(self).set(payload).get_result::<Self>(conn)?)
    }
}

impl Creatable<Template> for NewTemplate {
    fn create(&self, conn: &Connection) -> MainmanResult<Template> {
        Ok(diesel::insert_into(template::table)
            .values(self)
            .get_result::<Template>(conn)?)
    }
}

use crate::schema::*;

table! {
    entity_overview (uuid) {
        uuid -> Uuid,
        organisation -> Int8,
        pending_requests -> Int8,
        unfinished_requests -> Int8,
        finished_requests -> Int8,
    }
}

joinable!(entity -> entity_overview (uuid));

allow_tables_to_appear_in_same_query!(entity, entity_overview);

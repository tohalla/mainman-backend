table! {
    account (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        password -> Bytea,
    }
}

table! {
    account_role (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        organisation -> Int4,
        name -> Varchar,
        rights -> Jsonb,
    }
}

table! {
    entity (hash) {
        hash -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        name -> Varchar,
        description -> Nullable<Text>,
        organisation -> Int4,
    }
}

table! {
    maintainer (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        organisation -> Int4,
        account -> Nullable<Int4>,
        details -> Nullable<Jsonb>,
    }
}

table! {
    maintainer_entity (entity, maintainer) {
        entity -> Uuid,
        maintainer -> Int4,
        organisation -> Int4,
    }
}

table! {
    maintenance_event (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        resolved_at -> Nullable<Timestamp>,
        entity -> Uuid,
        description -> Nullable<Text>,
    }
}

table! {
    maintenance_task (hash) {
        hash -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        accepted_at -> Nullable<Timestamp>,
        resolved_at -> Nullable<Timestamp>,
        maintenance_event -> Int4,
        maintainer -> Int4,
        is_available -> Bool,
    }
}

table! {
    organisation (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        name -> Varchar,
        organisation_identifier -> Nullable<Varchar>,
        locale -> Varchar,
        admin_account -> Int4,
        plan -> Nullable<Int4>,
    }
}

table! {
    organisation_account (account, organisation) {
        account -> Int4,
        organisation -> Int4,
        account_role -> Int4,
    }
}

table! {
    plan (id) {
        id -> Int4,
        name -> Text,
        entities -> Nullable<Int4>,
        maintainers -> Nullable<Int4>,
        accounts -> Nullable<Int4>,
        is_public -> Bool,
    }
}

table! {
    refresh_token (token) {
        created_at -> Nullable<Timestamp>,
        account_id -> Int4,
        token -> Uuid,
        authentication_token -> Nullable<Text>,
    }
}

joinable!(account_role -> organisation (organisation));
joinable!(entity -> organisation (organisation));
joinable!(maintainer -> account (account));
joinable!(maintainer -> organisation (organisation));
joinable!(maintenance_event -> entity (entity));
joinable!(maintenance_task -> maintainer (maintainer));
joinable!(maintenance_task -> maintenance_event (maintenance_event));
joinable!(organisation -> account (admin_account));
joinable!(organisation -> plan (plan));
joinable!(organisation_account -> account (account));
joinable!(organisation_account -> account_role (account_role));
joinable!(organisation_account -> organisation (organisation));
joinable!(refresh_token -> account (account_id));

allow_tables_to_appear_in_same_query!(
    account,
    account_role,
    entity,
    maintainer,
    maintainer_entity,
    maintenance_event,
    maintenance_task,
    organisation,
    organisation_account,
    plan,
    refresh_token,
);

table! {
    account (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        password -> Bytea,
        stripe_customer -> Nullable<Text>,
    }
}

table! {
    account_role (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        organisation -> Nullable<Int8>,
        name -> Varchar,
        rights -> Jsonb,
    }
}

table! {
    entity (uuid) {
        uuid -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        name -> Varchar,
        description -> Nullable<Text>,
        organisation -> Int8,
    }
}

table! {
    maintainer (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        organisation -> Int8,
        account -> Nullable<Int8>,
        details -> Nullable<Jsonb>,
    }
}

table! {
    maintainer_entity (entity, maintainer) {
        entity -> Uuid,
        maintainer -> Int8,
        organisation -> Int8,
    }
}

table! {
    maintenance_event (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        resolved_at -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        maintenance_request -> Nullable<Int8>,
    }
}

table! {
    maintenance_request (id) {
        id -> Int8,
        created_at -> Timestamp,
        created_by -> Nullable<Int8>,
        entity -> Uuid,
        description -> Nullable<Text>,
        maintenance_trigger -> Nullable<Uuid>,
    }
}

table! {
    maintenance_task (uuid) {
        uuid -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        accepted_at -> Nullable<Timestamp>,
        resolved_at -> Nullable<Timestamp>,
        maintenance_event -> Int8,
        maintainer -> Int8,
        is_available -> Bool,
    }
}

table! {
    maintenance_trigger (uuid) {
        uuid -> Uuid,
        created_at -> Timestamp,
        entity -> Uuid,
        template -> Nullable<Int8>,
    }
}

table! {
    organisation (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        name -> Varchar,
        organisation_identifier -> Nullable<Varchar>,
        locale -> Varchar,
        plan -> Int4,
    }
}

table! {
    organisation_account (account, organisation) {
        account -> Int8,
        organisation -> Int8,
        account_role -> Nullable<Int8>,
    }
}

table! {
    organisation_invite (uuid) {
        uuid -> Uuid,
        organisation -> Int8,
        email -> Text,
        created_at -> Timestamp,
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
        stripe_product -> Nullable<Text>,
        stripe_price -> Nullable<Jsonb>,
    }
}

table! {
    refresh_token (token) {
        created_at -> Nullable<Timestamp>,
        account_id -> Int8,
        token -> Uuid,
        authentication_token -> Nullable<Text>,
    }
}

table! {
    template (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        organisation -> Nullable<Int8>,
        name -> Nullable<Varchar>,
        content -> Text,
        is_draft -> Bool,
        template_type -> Int4,
    }
}

table! {
    template_type (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(account_role -> organisation (organisation));
joinable!(entity -> organisation (organisation));
joinable!(maintainer -> account (account));
joinable!(maintainer -> organisation (organisation));
joinable!(maintainer_entity -> entity (entity));
joinable!(maintainer_entity -> maintainer (maintainer));
joinable!(maintenance_event -> maintenance_request (maintenance_request));
joinable!(maintenance_request -> account (created_by));
joinable!(maintenance_request -> entity (entity));
joinable!(maintenance_request -> maintenance_trigger (maintenance_trigger));
joinable!(maintenance_task -> maintainer (maintainer));
joinable!(maintenance_task -> maintenance_event (maintenance_event));
joinable!(maintenance_trigger -> entity (entity));
joinable!(maintenance_trigger -> template (template));
joinable!(organisation -> plan (plan));
joinable!(organisation_account -> account (account));
joinable!(organisation_account -> account_role (account_role));
joinable!(organisation_account -> organisation (organisation));
joinable!(organisation_invite -> organisation (organisation));
joinable!(refresh_token -> account (account_id));
joinable!(template -> organisation (organisation));
joinable!(template -> template_type (template_type));

allow_tables_to_appear_in_same_query!(
    account,
    account_role,
    entity,
    maintainer,
    maintainer_entity,
    maintenance_event,
    maintenance_request,
    maintenance_task,
    maintenance_trigger,
    organisation,
    organisation_account,
    organisation_invite,
    plan,
    refresh_token,
    template,
    template_type,
);

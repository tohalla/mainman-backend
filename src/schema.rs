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
    organisation (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        name -> Varchar,
        organisation_identifier -> Nullable<Varchar>,
        locale -> Varchar,
        admin_account -> Int4,
    }
}

table! {
    organisation_account (uuid) {
        uuid -> Uuid,
        account -> Int4,
        organisation -> Int4,
        account_role -> Int4,
    }
}

joinable!(account_role -> organisation (organisation));
joinable!(organisation -> account (admin_account));
joinable!(organisation_account -> account (account));
joinable!(organisation_account -> account_role (account_role));
joinable!(organisation_account -> organisation (organisation));

allow_tables_to_appear_in_same_query!(
    account,
    account_role,
    organisation,
    organisation_account,
);

table! {
    accounts (id) {
        id -> Varchar,
        email -> Varchar,
        username -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    messages (id) {
        id -> Varchar,
        from -> Varchar,
        to -> Varchar,
        content_type -> Int4,
        metadata -> Json,
        text -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    operations (revision) {
        revision -> Int4,
        op_type -> Int4,
        source -> Varchar,
        destination -> Array<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    profiles (id) {
        id -> Varchar,
        display_name -> Varchar,
        status_message -> Nullable<Varchar>,
        icon_path -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    messages,
    operations,
    profiles,
);

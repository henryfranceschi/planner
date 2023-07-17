// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        description -> Nullable<Text>,
        image_uri -> Nullable<Text>,
        cover_uri -> Nullable<Text>,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email_address -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(profiles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    profiles,
    users,
);

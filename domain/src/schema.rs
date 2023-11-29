// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        description -> Nullable<Text>,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        archived -> Bool,
        client_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    projects_users (project_id, user_id) {
        project_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        deadline -> Nullable<Timestamptz>,
        done -> Bool,
        project_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tasks_users (task_id, user_id) {
        task_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamptz,
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
diesel::joinable!(projects -> clients (client_id));
diesel::joinable!(projects_users -> projects (project_id));
diesel::joinable!(projects_users -> users (user_id));
diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(tasks_users -> tasks (task_id));
diesel::joinable!(tasks_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    profiles,
    projects,
    projects_users,
    tasks,
    tasks_users,
    users,
);

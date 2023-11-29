create table "projects"
(
    "id"                uuid        not null default gen_random_uuid(),
    "name"              text        not null,
    "description"       text,
    "archived"          boolean     not null default false,

    -- Relations
    "client_id"         uuid,

    -- Timestamps
    "created_at"        timestamptz not null default now(),
    "updated_at"        timestamptz,

    -- Constraints
    primary key ("id"),

    foreign key ("client_id")
        references "clients" ("id")
        on delete cascade
);

select trigger_updated_at('"projects"');

-- Junction table for users that are members of a project
create table "projects_users"
(
    "project_id"    uuid        not null,
    "user_id"       uuid        not null,

    -- As this is a junction table with no columns other than foreign keys we
    -- don't need an updated_at column for this table
    "created_at"    timestamptz not null default now(),

    -- Constraints
    primary key("project_id", "user_id"),

    foreign key("project_id")
        references "projects"("id")
        on delete cascade,

    foreign key("user_id")
        references "users"("id")
        on delete cascade
);

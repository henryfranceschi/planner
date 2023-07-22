create table "tasks"
(
    "id"            uuid            not null default gen_random_uuid(),
    "name"          text            not null,
    "description"   text,
    "deadline"      timestamptz,
    "done"          boolean         not null default false,

    -- Relations
    "project_id"    uuid            not null,

    -- Timestamps
    "created_at"    timestamptz     not null default now(),
    "updated_at"    timestamptz,

    -- Constraints
    primary key("id"),

    foreign key("project_id")
        references "projects"("id")
        on delete cascade
);

select trigger_updated_at('"tasks"');

-- Table for users that have been assigned to a task
create table "tasks_users"
(
    "task_id"       uuid        not null,
    "user_id"       uuid        not null,

    "created_at"    timestamptz not null default now(),

    -- Constraints
    primary key("task_id", "user_id"),

    foreign key("task_id")
        references "tasks"("id")
        on delete cascade,

    foreign key("user_id")
        references "users"("id")
        on delete cascade
);

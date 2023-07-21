create table "profiles"
(
    "id"            uuid        not null default gen_random_uuid(),
    "first_name"    text        not null,
    "last_name"     text        not null,
    "description"   text,
    
    -- Relations
    "user_id"       uuid        not null,

    -- Timestamps
    "created_at"    timestamptz not null default now(),
    "updated_at"    timestamptz,

    -- Constraints
    primary key ("id"),

    foreign key ("user_id")
        references "users"("id")
        on delete cascade
);

select trigger_updated_at('"profiles"');

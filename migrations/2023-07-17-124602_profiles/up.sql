create table "profiles"
(
    "id"            uuid        not null default uuid_generate_v4(),
    "first_name"    text        not null,
    "last_name"     text        not null,
    "description"   text,
    "image_uri"     text,
    "cover_uri"     text,
    
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

select trigger_updated_at("profiles");

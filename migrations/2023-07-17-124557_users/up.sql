create table "users"
(
    "id"            uuid        not null default uuid_generate_v4(),
    "email_address" text        not null,
    "password_hash" text        not null,

    -- Timestamps
    "created_at"    timestamptz not null default now(),
    "updated_at"    timestamptz,

    -- Constraints
    primary key("id"),

    unique("email_address")
);

select trigger_updated_at("users");

create table "clients"
(
    "id"            uuid    not null default gen_random_uuid(),
    "name"          text    not null,
    "description"   text,

    -- Timestamps
    "created_at"    timestamptz not null default now(),
    "updated_at"    timestamptz,

    -- Constraints
    primary key ("id")
);

select trigger_updated_at('"clients"');

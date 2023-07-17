create or replace function set_updated_at()
    returns trigger
as $$
begin
    new.updated_at = now();
    return new;
end;
$$ language plpgsql;

-- Adds a trigger to the table which executes the set_updated_at function before every update.
create or replace function trigger_updated_at(table_name regclass)
    returns void as
$$
begin
    execute format
    (
        $f$
            create trigger set_updated_at
            before update
            on %s
            for each row
            when (old is distinct from new)
            execute function set_updated_at();
        $f$,
        table_name
    );
end;
$$ language plpgsql;

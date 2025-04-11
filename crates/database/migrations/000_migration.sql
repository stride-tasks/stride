-- SQL: up

CREATE TABLE migration_table (
    migration_id INTEGER PRIMARY KEY,
    migration_filename TEXT NOT NULL,
    migration_sql_up TEXT NOT NULL,
    migration_sql_down TEXT NOT NULL
) STRICT;

-- SQL: down

DROP TABLE IF EXISTS migration_table;

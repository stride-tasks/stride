use super::Migration;

const SQL: &str = r####"
CREATE TABLE migration_table (
    `id` INTEGER PRIMARY KEY,
    `sql` TEXT NOT NULL
) STRICT;
"####;

pub(super) struct InitialMigration;

impl Migration for InitialMigration {
    fn sql(&self) -> &str {
        SQL
    }
}

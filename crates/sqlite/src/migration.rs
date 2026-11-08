use rusqlite::{Connection, OptionalExtension, Transaction};

use crate::Result;

const CREATE_TABLE_SQL: &str = "
-------------------------------------------
--              migration                --
-------------------------------------------

CREATE TABLE IF NOT EXISTS migration_table (
    `id` INTEGER PRIMARY KEY,
    `sql` TEXT NOT NULL
) STRICT;
";
const INSERT_MIGRATION_SQL: &str = "INSERT INTO migration_table (id, sql) VALUES (?1, ?2)";
const SELECT_LAST_MIGRATION_INDEX: &str = "SELECT id FROM migration_table ORDER BY id DESC LIMIT 1";

/// A single step in a database's schema history.
///
/// The position of a migration in the list given to [`apply_migrations`] is its identity,
/// so migrations must only ever be appended.
pub trait Migration {
    /// The SQL to execute, it runs inside the transaction that applies all the migrations.
    fn sql(&self) -> &str;

    fn apply(&self, transaction: &mut Transaction<'_>) -> Result<()> {
        transaction.execute_batch(self.sql())?;
        Ok(())
    }

    fn pre_apply(&self, transaction: &mut Transaction<'_>) -> Result<()> {
        let _ = transaction;
        Ok(())
    }

    fn post_apply(&self, transaction: &mut Transaction<'_>) -> Result<()> {
        let _ = transaction;
        Ok(())
    }
}

/// Applies all the migrations that were not applied before, in order.
pub fn apply_migrations(connection: &mut Connection, migrations: &[&dyn Migration]) -> Result<()> {
    let mut transaction = connection.transaction()?;
    transaction.execute_batch(CREATE_TABLE_SQL)?;

    let next_migration_index = transaction
        .query_row(SELECT_LAST_MIGRATION_INDEX, (), |row| {
            row.get::<_, usize>(0)
        })
        .optional()?
        .map_or(0, |last| last + 1);

    for (index, migration) in migrations.iter().enumerate().skip(next_migration_index) {
        migration.pre_apply(&mut transaction)?;
        migration.apply(&mut transaction)?;
        transaction.execute(INSERT_MIGRATION_SQL, (index, migration.sql()))?;
        migration.post_apply(&mut transaction)?;
    }
    transaction.commit()?;
    Ok(())
}

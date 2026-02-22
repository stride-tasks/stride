use rusqlite::OptionalExtension;

use crate::{Database, Result};

mod v000_initial;

const MIGRATIONS: &[&dyn Migration] = &[&v000_initial::InitialMigration];

trait Migration {
    fn sql(&self) -> &str;

    fn apply(&self, db: &mut Database) -> Result<()> {
        db.connection.execute_batch(self.sql())?;
        Ok(())
    }

    fn pre_apply(&self, db: &mut Database) -> Result<()> {
        let _ = db;
        Ok(())
    }
    fn post_apply(&self, db: &mut Database) -> Result<()> {
        let _ = db;
        Ok(())
    }
}

fn table_exists(db: &Database, table_name: &str) -> Result<bool, rusqlite::Error> {
    const QUERY: &str = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?";
    let mut stmt = db.connection.prepare(QUERY)?;
    let mut rows = stmt.query([table_name])?;

    if let Some(row) = rows.next()? {
        let count: i32 = row.get(0)?;
        Ok(count > 0)
    } else {
        Ok(false)
    }
}

const TABLE_NAME: &str = "migration_table";
const INSERT_MIGRATION_SQL: &str = "INSERT INTO migration_table (id, sql) VALUES (?1, ?2)";
const SELECT_LAST_MIGRATION_INDEX: &str = "SELECT id FROM migration_table ORDER BY id DESC LIMIT 1";

pub(crate) fn apply_migrations(db: &mut Database) -> Result<()> {
    let mut last_migration_index: usize = 0;
    if table_exists(db, TABLE_NAME)? {
        last_migration_index = db
            .connection
            .query_row(SELECT_LAST_MIGRATION_INDEX, (), |row| row.get(0))
            .optional()?
            .unwrap_or(0)
            + 1;
    }

    for (index, migration) in MIGRATIONS.iter().enumerate().skip(last_migration_index) {
        migration.pre_apply(db)?;
        migration.apply(db)?;
        db.connection
            .execute(INSERT_MIGRATION_SQL, (index, migration.sql()))?;
        migration.post_apply(db)?;
    }
    Ok(())
}

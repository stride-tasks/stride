use stride_sqlite::{Migration, StorageMigration};

use crate::{Database, Result};

mod v000_initial;

const MIGRATIONS: &[&dyn Migration] = &[&v000_initial::InitialMigration, &StorageMigration];

pub(crate) fn apply_migrations(db: &mut Database) -> Result<()> {
    stride_sqlite::apply_migrations(&mut db.connection, MIGRATIONS)?;
    Ok(())
}

use indoc::indoc;
use rusqlite::{Connection, OptionalExtension, Transaction};
use stride_api::{ConfigValue, StorageItem};

use std::path::Path;

use crate::{Error, Migration, Result, apply_migrations};

#[cfg(test)]
mod tests;

const SQL: &str = r"
BEGIN TRANSACTION;

CREATE TABLE storage_table (
    `key` TEXT NOT NULL PRIMARY KEY CHECK (length(`key`) != 0),
    `type` TEXT NOT NULL,
    `value` BLOB NOT NULL
) WITHOUT ROWID, STRICT;

COMMIT;
";

/// Creates the table used by [`Storage`].
///
/// Add it to the migration list of any database that should have a key-value storage.
#[derive(Debug, Clone, Copy)]
pub struct StorageMigration;

impl Migration for StorageMigration {
    fn sql(&self) -> &str {
        SQL
    }
}

/// Checks that the key is a '.' separated list of non-empty names,
/// where a name is made of ASCII alphanumerics, '-' or '_'.
pub fn validate_key(key: &str) -> Result<()> {
    let is_valid = key.split('.').all(|name| {
        !name.is_empty()
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    });
    if is_valid {
        Ok(())
    } else {
        Err(Error::InvalidKey { key: key.into() })
    }
}

/// Key-value storage over a connection to a database that applied [`StorageMigration`].
#[derive(Debug)]
pub struct StorageTransaction<'a> {
    transaction: Transaction<'a>,
}

impl<'a> StorageTransaction<'a> {
    #[must_use]
    pub fn new(transaction: Transaction<'a>) -> Self {
        Self { transaction }
    }

    /// Lists all the stored items, ordered by key.
    pub fn list(&self) -> Result<Vec<StorageItem>> {
        let mut sql = self.transaction.prepare_cached(indoc! {"
            SELECT key, type, value FROM storage_table ORDER BY key
        "})?;
        let rows = sql.query_map((), Self::row_to_item)?;
        rows.map(|row| row?).collect()
    }

    /// Gets the items of the given keys, keys that are not stored are omitted.
    pub fn get<K: AsRef<str>>(&self, keys: &[K]) -> Result<Vec<StorageItem>> {
        let mut sql = self.transaction.prepare_cached(indoc! {"
            SELECT key, type, value FROM storage_table WHERE key = ?1
        "})?;

        let mut items = Vec::with_capacity(keys.len());
        for key in keys {
            let key = key.as_ref();
            validate_key(key)?;
            if let Some(item) = sql.query_row((key,), Self::row_to_item).optional()? {
                items.push(item?);
            }
        }
        Ok(items)
    }

    /// Adds the key, or changes its value if it already exists.
    pub fn set(&self, key: &str, value: &ConfigValue) -> Result<()> {
        validate_key(key)?;
        let mut sql = self.transaction.prepare_cached(indoc! {"
            INSERT INTO storage_table (key, type, value) VALUES (?1, ?2, ?3)
            ON CONFLICT (key) DO UPDATE SET type = excluded.type, value = excluded.value
        "})?;
        sql.execute((key, value.as_type_name(), value.as_value_boxed_slice()))?;
        Ok(())
    }

    /// Removes the given keys atomically, returns the keys that were actually removed.
    pub fn remove<K: AsRef<str>>(&self, keys: &[K]) -> Result<Vec<Box<str>>> {
        for key in keys {
            validate_key(key.as_ref())?;
        }

        let mut removed = Vec::new();
        {
            let mut sql = self
                .transaction
                .prepare_cached("DELETE FROM storage_table WHERE key = ?1")?;
            for key in keys {
                let key = key.as_ref();
                if sql.execute((key,))? != 0 {
                    removed.push(key.into());
                }
            }
        }
        Ok(removed)
    }

    fn row_to_item(row: &rusqlite::Row<'_>) -> rusqlite::Result<Result<StorageItem>> {
        let key = row.get::<_, Box<str>>("key")?;
        let type_name = row.get::<_, Box<str>>("type")?;
        let bytes = row.get::<_, Vec<u8>>("value")?;

        Ok(ConfigValue::from_type_name_and_bytes(&type_name, &bytes)
            .map(|value| StorageItem {
                key: key.clone(),
                value,
            })
            .map_err(|source| Error::InvalidValue { key, source }))
    }
}

/// A standalone sqlite database that only contains the key-value storage.
///
/// Used for storage that isn't owned by a more specific database, like the global storage.
#[derive(Debug)]
pub struct StorageDatabase {
    connection: Connection,
}

impl StorageDatabase {
    /// Opens (creating it if needed) the database at the given path and applies migrations.
    pub fn open(path: &Path) -> Result<Self> {
        Self::init(Connection::open(path)?)
    }

    pub fn open_in_memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(mut connection: Connection) -> Result<Self> {
        apply_migrations(&mut connection, &[&StorageMigration])?;
        Ok(Self { connection })
    }

    pub fn transaction(&mut self) -> Result<Transaction<'_>> {
        Ok(self.connection.transaction()?)
    }
}

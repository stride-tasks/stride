use indoc::indoc;
use rusqlite::{Connection, OptionalExtension, Transaction};
use stride_api::{ConfigValue, StorageItem};

use std::path::Path;

use crate::{Error, Migration, Result, apply_migrations};

#[cfg(test)]
mod tests;

const SQL: &str = r"
CREATE TABLE storage_namespace_table (
    `id` INTEGER NOT NULL PRIMARY KEY,
    `name` TEXT NOT NULL UNIQUE CHECK (length(`name`) != 0)
) STRICT;

CREATE TABLE storage_item_table (
    `namespace_id` INTEGER NOT NULL,
    `key` TEXT NOT NULL CHECK (length(`key`) != 0),
    `type` TEXT NOT NULL,
    `value` BLOB NOT NULL,

    PRIMARY KEY (`namespace_id`, `key`),
    FOREIGN KEY (`namespace_id`) REFERENCES storage_namespace_table(`id`) ON DELETE CASCADE
) WITHOUT ROWID, STRICT;

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

fn is_valid_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

/// Splits a `group:key` query into its group and key.
///
/// Both parts are '.' separated lists of non-empty names,
/// where a name is made of ASCII alphanumerics, '-' or '_'.
pub fn parse_key(query: &str) -> Result<(&str, &str)> {
    query
        .split_once(':')
        .filter(|(group, key)| {
            group.split('.').all(is_valid_name) && key.split('.').all(is_valid_name)
        })
        .ok_or_else(|| Error::InvalidKey { key: query.into() })
}

/// Checks that the key is a valid `group:key` query.
pub fn validate_key(key: &str) -> Result<()> {
    parse_key(key).map(|_| ())
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

    /// Lists all the stored items, ordered by group and key.
    pub fn list(&self) -> Result<Vec<StorageItem>> {
        self.list_in_group(None)
    }

    /// Lists the stored items in one group, ordered by key.
    pub fn list_group(&self, group: &str) -> Result<Vec<StorageItem>> {
        let query = format!("{group}:_");
        parse_key(&query)?;
        self.list_in_group(Some(group))
    }

    fn list_in_group(&self, group: Option<&str>) -> Result<Vec<StorageItem>> {
        let mut sql = self.transaction.prepare_cached(indoc! {"
            SELECT n.name || ':' || i.key AS key, i.type, i.value
            FROM storage_item_table i
            JOIN storage_namespace_table n ON n.id = i.namespace_id
            WHERE (?1 IS NULL OR n.name = ?1)
            ORDER BY n.name, i.key
        "})?;
        let rows = sql.query_map([group], Self::row_to_item)?;
        rows.map(|row| row?).collect()
    }

    /// Gets the items of the given `group:key` queries, keys that are not stored are omitted.
    pub fn get<K: AsRef<str>>(&self, keys: &[K]) -> Result<Vec<StorageItem>> {
        let mut sql = self.transaction.prepare_cached(indoc! {"
            SELECT ?1 || ':' || i.key AS key, i.type, i.value
            FROM storage_item_table i
            JOIN storage_namespace_table n ON n.id = i.namespace_id
            WHERE n.name = ?1 AND i.key = ?2
        "})?;

        let mut items = Vec::with_capacity(keys.len());
        for key in keys {
            let (group, key) = parse_key(key.as_ref())?;
            if let Some(item) = sql.query_row((group, key), Self::row_to_item).optional()? {
                items.push(item?);
            }
        }
        Ok(items)
    }

    /// Adds the key, or changes its value if it already exists.
    pub fn set(&self, query: &str, value: &ConfigValue) -> Result<()> {
        let (group, key) = parse_key(query)?;
        self.transaction
            .prepare_cached(indoc! {"
            INSERT INTO storage_namespace_table (name) VALUES (?1)
            ON CONFLICT (name) DO NOTHING
        "})?
            .execute((group,))?;

        let mut sql = self.transaction.prepare_cached(indoc! {"
            INSERT INTO storage_item_table (namespace_id, key, type, value)
            VALUES ((SELECT id FROM storage_namespace_table WHERE name = ?1), ?2, ?3, ?4)
            ON CONFLICT (namespace_id, key) DO UPDATE SET type = excluded.type, value = excluded.value
        "})?;
        sql.execute((
            group,
            key,
            value.as_type_name(),
            value.as_value_boxed_slice(),
        ))?;
        Ok(())
    }

    /// Removes the given keys atomically, returns the keys that were actually removed.
    ///
    /// Groups that become empty are removed as well.
    pub fn remove<K: AsRef<str>>(&self, keys: &[K]) -> Result<Vec<Box<str>>> {
        let keys = keys
            .iter()
            .map(|key| parse_key(key.as_ref()).map(|(group, name)| (key.as_ref(), group, name)))
            .collect::<Result<Vec<_>>>()?;

        let mut removed = Vec::new();
        {
            let mut sql = self.transaction.prepare_cached(indoc! {"
                DELETE FROM storage_item_table
                WHERE key = ?2
                  AND namespace_id = (SELECT id FROM storage_namespace_table WHERE name = ?1)
            "})?;
            for (query, group, key) in keys {
                if sql.execute((group, key))? != 0 {
                    removed.push(query.into());
                }
            }
        }

        self.transaction.execute(
            indoc! {"
                DELETE FROM storage_namespace_table
                WHERE id NOT IN (SELECT DISTINCT namespace_id FROM storage_item_table)
            "},
            (),
        )?;
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

    pub fn commit(self) -> Result<()> {
        self.transaction.commit()?;
        Ok(())
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

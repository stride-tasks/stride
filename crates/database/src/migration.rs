use rusqlite::{Connection, OptionalExtension};

use crate::Result;

#[derive(Debug)]
pub(crate) struct Migration<'a> {
    pub(crate) index: usize,
    pub(crate) filename: &'a str,
    pub(crate) sql: &'a str,
}

const fn parse_index(input: &[u8]) -> usize {
    let mut result = 0;
    let mut i = 0;
    while i < input.len() {
        let ch = input[i];
        assert!(ch.is_ascii_digit());

        result = result * 10 + (ch - b'0') as usize;

        i += 1;
    }
    result
}

pub(crate) const fn parse_migration(
    filename: &'static str,
    content: &'static str,
) -> Migration<'static> {
    assert!(filename.is_ascii(), "migration filename must be ascii");
    assert!(content.is_ascii(), "migration content must be ascii");

    let bytes = filename.as_bytes();

    let (index, rest) = bytes.split_at(3);
    let separator = rest[0];

    assert!(separator == b'_');

    let index = parse_index(index);
    let sql = content;

    Migration {
        index,
        filename,
        sql,
    }
}

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool, rusqlite::Error> {
    const QUERY: &str = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?";
    let mut stmt = conn.prepare(QUERY)?;
    let mut rows = stmt.query([table_name])?;

    if let Some(row) = rows.next()? {
        let count: i32 = row.get(0)?;
        Ok(count > 0)
    } else {
        Ok(false)
    }
}

const TABLE_NAME: &str = "migration_table";
const INSERT_MIGRATION_SQL: &str = r"
INSERT INTO migration_table (
    id,
    filename,
    sql
) VALUES (?1, ?2, ?3)";
const SELECT_LAST_MIGRATION_INDEX: &str =
    "SELECT id FROM migration_table ORDER BY ROWID DESC LIMIT 1";

#[derive(Debug, Clone, Copy)]
pub(crate) struct Migrations {
    pub(crate) migrations: &'static [Migration<'static>],
}

impl Migrations {
    pub(crate) fn apply(&self, conn: &mut Connection) -> Result<()> {
        let mut last_migration_index: usize = 0;
        if table_exists(conn, TABLE_NAME)? {
            last_migration_index = conn
                .query_row(SELECT_LAST_MIGRATION_INDEX, (), |row| row.get(0))
                .optional()?
                .unwrap_or(0)
                + 1;
        }

        for migration in self.migrations.iter().skip(last_migration_index) {
            let sql = format!("BEGIN;\n\n{}\n\nCOMMIT;\n", migration.sql);
            conn.execute_batch(&sql)?;
            conn.execute(
                INSERT_MIGRATION_SQL,
                (migration.index, migration.filename, migration.sql),
            )?;
        }
        Ok(())
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! migrations {
    [$($filename:expr,)+] => {
        pub(crate) const MIGRATIONS: $crate::migration::Migrations = $crate::migration::Migrations {
            migrations: &[
                $(
                    $crate::migration::parse_migration(
                        $filename,
                        include_str!(concat!("../migrations/", $filename))
                    ),
                )+
            ],
        };
    };
}

migrations!["000_migration.sql", "001_tasks.sql",];

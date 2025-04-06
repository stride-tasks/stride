use rusqlite::{Connection, OptionalExtension};

use crate::{Error, Result};

#[derive(Debug)]
pub(crate) struct Migration<'a> {
    pub(crate) index: usize,
    pub(crate) filename: &'a str,
    pub(crate) sql_up: &'a str,
    pub(crate) sql_down: &'a str,
}

const fn ignore_case_starts_with(mut a: &[u8], mut b: &[u8]) -> bool {
    if a.len() < b.len() {
        return false;
    }

    while let ([first_a, rest_a @ ..], [first_b, rest_b @ ..]) = (a, b) {
        if first_a.eq_ignore_ascii_case(first_b) {
            a = rest_a;
            b = rest_b;
        } else {
            return false;
        }
    }

    true
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

const fn parse_content(input: &[u8]) -> (&str, &str) {
    assert!(input.is_ascii(), "migration content must be ascii");

    assert!(
        ignore_case_starts_with(input, b"-- SQL: up"),
        "migration must start with up section"
    );

    let mut i = 0;
    while i < input.len() {
        let (_, rest) = input.split_at(i);
        if ignore_case_starts_with(rest, b"-- SQL: down") {
            break;
        }
        i += 1;
    }

    let (sql_up, sql_down) = input.split_at(i);

    // SAFETY: Ascii is valid utf-8
    unsafe {
        (
            std::str::from_utf8_unchecked(sql_up),
            std::str::from_utf8_unchecked(sql_down),
        )
    }
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
    let (sql_up, sql_down) = parse_content(content.as_bytes());

    Migration {
        index,
        filename,
        sql_up,
        sql_down,
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
    migration_id,
    migration_filename,
    migration_sql_up,
    migration_sql_down
) VALUES (?1, ?2, ?3, ?4)";
const SELECT_LAST_MIGRATION_INDEX: &str =
    "SELECT migration_id FROM migration_table ORDER BY ROWID DESC LIMIT 1";
const DELETE_MIGRATION_WITH_INDEX: &str = "DELETE FROM migration_table WHERE migration_id=?1";

#[derive(Debug)]
pub struct Migrations {
    pub(crate) migrations: &'static [Migration<'static>],
}

impl Migrations {
    pub fn apply(&self, conn: &mut Connection) -> Result<()> {
        let mut last_migration_index: usize = 0;
        if table_exists(conn, TABLE_NAME)? {
            last_migration_index = conn
                .query_row(SELECT_LAST_MIGRATION_INDEX, (), |row| row.get(0))
                .optional()?
                .unwrap_or(0)
                + 1;
        }

        for migration in self.migrations.iter().skip(last_migration_index) {
            let sql = format!("BEGIN;\n\n{}\n\nCOMMIT;\n", migration.sql_up);
            conn.execute_batch(&sql)?;
            conn.execute(
                INSERT_MIGRATION_SQL,
                (
                    migration.index,
                    migration.filename,
                    migration.sql_up,
                    migration.sql_down,
                ),
            )?;
        }
        Ok(())
    }

    pub fn unapply(&self, conn: &mut Connection) -> Result<()> {
        for (i, migration) in self.migrations.iter().enumerate().rev() {
            let sql = format!("BEGIN;\n\n{}\n\nCOMMIT;\n", migration.sql_down);

            conn.execute_batch(&sql)?;

            if i != 0 {
                // Last migration deletes migration table
                conn.execute(DELETE_MIGRATION_WITH_INDEX, (migration.index,))?;
            }
        }
        Ok(())
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! migrations {
    [$($filename:expr,)+] => {
        const MIGRATIONS: $crate::migration::Migrations = $crate::migration::Migrations {
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

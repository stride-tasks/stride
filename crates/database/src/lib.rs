mod error;
mod migration;

pub use error::{Error, Result};
pub use migration::Migrations;
use rusqlite::Connection;

migrations!["000_migration.sql", "001_tasks.sql",];

pub fn apply_migrations(conn: &mut Connection) -> Result<()> {
    // let mut conn = Connection::open("./test.sqlite")?;
    conn.pragma_update(None, "journal_mode", "WAL")?;

    MIGRATIONS.apply(conn)?;

    // conn.execute(
    //     "INSERT INTO task_table (task_uuid, task_title) VALUES (?1, ?2)",
    //     (&[0x0fu8; 16], "Test"),
    // )?;

    // conn.execute(
    //     "INSERT INTO annotation_table (annotation_task_id, annotation_text) VALUES (?1, ?2)",
    //     (1, "Annotation text happy :D"),
    // )?;

    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(Person {
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;

    // MIGRATIONS.unapply(conn)?;

    Ok(())
}

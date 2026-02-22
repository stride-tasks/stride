use super::Migration;

const SQL: &str = r"
BEGIN TRANSACTION;

CREATE TABLE migration_table (
    `id` INTEGER PRIMARY KEY,
    `sql` TEXT NOT NULL
) STRICT;


CREATE TABLE IF NOT EXISTS task_table (
    `id` BLOB PRIMARY KEY, -- UUID
    `title` TEXT,
    `entry` INTEGER,
    `status` INTEGER NOT NULL DEFAULT 0, -- Default: pending
    `priority` INTEGER,
    `project` TEXT,
    `modified` INTEGER,
    `due` INTEGER,
    `wait` INTEGER,

    FOREIGN KEY (project) REFERENCES project_table (id) ON DELETE SET NULL
) STRICT;

CREATE TABLE IF NOT EXISTS task_dependency_table (
    parent_task_id BLOB NOT NULL,
    child_task_id BLOB NOT NULL,

    FOREIGN KEY (parent_task_id) REFERENCES task_table (id) ON DELETE CASCADE,
    FOREIGN KEY (child_task_id) REFERENCES task_table (id) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS project_table (
    id TEXT PRIMARY KEY CHECK (length(id) != 0)
) STRICT;

CREATE TABLE IF NOT EXISTS tag_table (
    id TEXT PRIMARY KEY CHECK (length(id) != 0)
) STRICT;

CREATE TABLE IF NOT EXISTS task_tag_table (
    task_id BLOB NOT NULL,
    tag_id TEXT NOT NULL,

    PRIMARY KEY (task_id, tag_id),

    FOREIGN KEY (task_id) REFERENCES task_table (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tag_table (id) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS annotation_table (
    `id` INTEGER NOT NULL PRIMARY KEY,
    `task_id` BLOB NOT NULL,
    `entry` INTEGER NOT NULL,
    `description` TEXT NOT NULL,

    FOREIGN KEY (`task_id`) REFERENCES task_table (`id`) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS uda_table (
    `id` INTEGER NOT NULL PRIMARY KEY,
    `task_id` BLOB NOT NULL,
    `key` TEXT NOT NULL,
    `value` TEXT NOT NULL,

    FOREIGN KEY (`task_id`) REFERENCES task_table (`id`) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS operation_table (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    kind BLOB
) STRICT;

CREATE TABLE IF NOT EXISTS backend_table (
    `id` BLOB NOT NULL PRIMARY KEY,
    `name` TEXT NOT NULL CHECK (length(`name`) != 0),
    `enabled` INTEGER NOT NULL CHECK (`enabled` = 0 OR `enabled` = 1)
) STRICT;

CREATE TABLE IF NOT EXISTS backend_config_table (
    `backend_id` BLOB NOT NULL,

    `name` TEXT NOT NULL,
    `type` TEXT NOT NULL,
    `value` BLOB NOT NULL,

    FOREIGN KEY (`backend_id`) REFERENCES backend_table (`id`) ON DELETE CASCADE
) STRICT;

CREATE INDEX IF NOT EXISTS task_tag_table_task_id_idx ON task_tag_table (task_id);
CREATE INDEX IF NOT EXISTS task_tag_table_tag_id_idx ON task_tag_table (tag_id);

ANALYZE;

COMMIT;
";

pub(super) struct InitialMigration;

impl Migration for InitialMigration {
    fn sql(&self) -> &str {
        SQL
    }
}

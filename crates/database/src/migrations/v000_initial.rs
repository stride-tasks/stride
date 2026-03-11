use super::Migration;

const SQL: &str = r"
BEGIN TRANSACTION;

-------------------------------------------
--              migration                --
-------------------------------------------

CREATE TABLE migration_table (
    `id` INTEGER PRIMARY KEY,
    `sql` TEXT NOT NULL
) STRICT;



-------------------------------------------
--                tasks                  --
-------------------------------------------

CREATE TABLE task_table (
    `id` BLOB PRIMARY KEY, -- UUIDv7
    `title` TEXT,
    `entry` INTEGER,
    `status` INTEGER,
    `priority` INTEGER,
    `project` TEXT,
    `modified` INTEGER,
    `due` INTEGER,
    `wait` INTEGER,

    `tombstone` INTEGER NOT NULL DEFAULT 0
) WITHOUT ROWID, STRICT;

CREATE TABLE task_dependency_table (
    `parent_task_id` BLOB NOT NULL,
    `child_task_id` BLOB NOT NULL,

    PRIMARY KEY (`parent_task_id`, `child_task_id`),

    FOREIGN KEY (`parent_task_id`) REFERENCES task_table (id) ON DELETE CASCADE,
    FOREIGN KEY (`child_task_id`) REFERENCES task_table (id) ON DELETE CASCADE
) WITHOUT ROWID, STRICT;

CREATE TABLE project_table (
    `id` TEXT PRIMARY KEY,
    `description` TEXT,
    `tombstone` INTEGER NOT NULL DEFAULT 0
) WITHOUT ROWID, STRICT;

CREATE TABLE tag_table (
    `id` TEXT PRIMARY KEY,
    `description` TEXT,
    `tombstone` INTEGER NOT NULL DEFAULT 0
) WITHOUT ROWID, STRICT;

CREATE TABLE task_tag_table (
    `task_id` BLOB NOT NULL,
    `tag_id` TEXT NOT NULL,

    PRIMARY KEY (`task_id`, `tag_id`),

    FOREIGN KEY (`task_id`) REFERENCES task_table (id) ON DELETE CASCADE
) WITHOUT ROWID, STRICT;

CREATE TABLE annotation_table (
    `id` BLOB NOT NULL PRIMARY KEY, -- UUIDv7

    `task_id` BLOB,
    `entry` INTEGER,
    `text` TEXT,

    `tombstone` INTEGER NOT NULL DEFAULT 0,

    FOREIGN KEY (`task_id`) REFERENCES task_table (`id`) ON DELETE CASCADE
) WITHOUT ROWID, STRICT;

CREATE TABLE uda_table (
    `task_id` BLOB NOT NULL,
    `key` TEXT NOT NULL,
    `value` TEXT NOT NULL,

    PRIMARY KEY (`task_id`, `key`),

    FOREIGN KEY (`task_id`) REFERENCES task_table (`id`) ON DELETE CASCADE
) WITHOUT ROWID, STRICT;


CREATE INDEX task_tag_table_task_id_idx ON task_tag_table (task_id);
CREATE INDEX task_tag_table_tag_id_idx ON task_tag_table (tag_id);



-------------------------------------------
--                CRDT                   --
-------------------------------------------

CREATE TABLE actor_table (
    `id` BLOB NOT NULL PRIMARY KEY,
    `sequence` INTEGER NOT NULL DEFAULT 0,
    `timestamp_logical` INTEGER NOT NULL,
    `timestamp_counter` INTEGER NOT NULL
) WITHOUT ROWID, STRICT;

-- A change is a group of operations that were made together by a single actor. 
CREATE TABLE change_table (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,

    `actor_id` BLOB NOT NULL,
    `sequence` INTEGER NOT NULL,
    `timestamp_logical` INTEGER NOT NULL,
    `timestamp_counter` INTEGER NOT NULL,

    FOREIGN KEY (`actor_id`) REFERENCES actor_table (`id`) ON DELETE CASCADE
) STRICT;

-- An operation is the smallest unit of modification.
CREATE TABLE operation_table (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,

    `change_id` INTEGER NOT NULL,
    `row_id` BLOB NOT NULL, -- ID of the row affected by the operation.
    `type` INTEGER NOT NULL,
    `data` BLOB NOT NULL,

    FOREIGN KEY (`change_id`) REFERENCES change_table (`id`) ON DELETE CASCADE
) STRICT;

CREATE INDEX operation_table_row_id__type_idx ON operation_table (`row_id`, `type`);



-------------------------------------------
--               backend                 --
-------------------------------------------

CREATE TABLE backend_table (
    `id` BLOB NOT NULL PRIMARY KEY, -- UUIDv7
    `name` TEXT NOT NULL CHECK (length(`name`) != 0),
    `enabled` INTEGER NOT NULL CHECK (`enabled` = 0 OR `enabled` = 1)
) WITHOUT ROWID, STRICT;

CREATE TABLE backend_config_table (
    `id` INTEGER NOT NULL PRIMARY KEY,

    `backend_id` BLOB NOT NULL,

    `name` TEXT NOT NULL,
    `type` TEXT NOT NULL,
    `value` BLOB NOT NULL,

    FOREIGN KEY (`backend_id`) REFERENCES backend_table (`id`) ON DELETE CASCADE
) STRICT;


ANALYZE;

COMMIT;
";

pub(super) struct InitialMigration;

impl Migration for InitialMigration {
    fn sql(&self) -> &str {
        SQL
    }
}

-- NOTE: The ID column of most of tables has to be globally unique,
--       because the application is intented to be used in a distributed way,
---      where the clients don't know about each other.

CREATE TABLE IF NOT EXISTS task_table (
    "id" BLOB PRIMARY KEY, -- UUID
    "title" TEXT NOT NULL,
    "entry" INTEGER NOT NULL,
    "status" INTEGER NOT NULL DEFAULT 0, -- Default: pending
    "priority" INTEGER,
    "project" TEXT,
    "modified" INTEGER,
    "due" INTEGER,
    "wait" INTEGER,

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

CREATE INDEX task_tag_table_task_id_idx ON task_tag_table (task_id);
CREATE INDEX task_tag_table_tag_id_idx ON task_tag_table (tag_id);

CREATE TABLE IF NOT EXISTS annotation_table (
    "id" INTEGER PRIMARY KEY,
    "entry" INTEGER NOT NULL,
    "text" TEXT NOT NULL,
    "task_id" BLOB NOT NULL,

    FOREIGN KEY (task_id) REFERENCES task_table (id) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS uda_table (
    "id" INTEGER PRIMARY KEY,
    "namespace" TEXT,
    "key" TEXT NOT NULL,
    "value" BLOB,
    "task_id" BLOB NOT NULL,

    FOREIGN KEY (task_id) REFERENCES task_table (id) ON DELETE CASCADE
) STRICT;

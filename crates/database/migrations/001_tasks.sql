-- SQL: up

CREATE TABLE status_table (
    status_id INTEGER PRIMARY KEY,
    status_name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE priority_table (
    priority_id INTEGER PRIMARY KEY,
    priority_name TEXT NOT NULL UNIQUE,
    priority_urgency REAL NOT NULL DEFAULT 0.0
) STRICT;

CREATE TABLE project_table (
    project_id INTEGER PRIMARY KEY,
    project_name TEXT NOT NULL UNIQUE,
    project_urgency REAL NOT NULL DEFAULT 0.0
) STRICT;

CREATE TABLE task_table (
    task_id BLOB PRIMARY KEY CHECK (length(task_id) = 16),
    task_title TEXT NOT NULL,
    task_entry INTEGER NOT NULL DEFAULT (unixepoch('now')),
    task_status_id INTEGER NOT NULL DEFAULT 0,
    task_priority_id INTEGER,
    task_project_id INTEGER,
    task_modified INTEGER,
    task_due INTEGER,
    task_wait INTEGER,
    task_annotations BLOB NOT NULL DEFAULT (empty_blob()),
    task_depends BLOB NOT NULL DEFAULT (empty_blob())
    CHECK (mod(length(task_depends), 16) = 0),

    FOREIGN KEY (task_status_id) REFERENCES status_table (status_id)
    ON DELETE RESTRICT,

    FOREIGN KEY (task_priority_id) REFERENCES priority_table (priority_id)
    ON DELETE SET NULL,

    FOREIGN KEY (task_project_id) REFERENCES project_table (project_id)
    ON DELETE SET NULL
) STRICT;

CREATE TABLE uda_table (
    uda_id INTEGER PRIMARY KEY,
    uda_task_id BLOB NOT NULL,
    uda_namespace TEXT NOT NULL DEFAULT '',
    uda_key TEXT NOT NULL,
    uda_value TEXT NOT NULL DEFAULT '',

    FOREIGN KEY (uda_task_id) REFERENCES task_table (task_id) ON DELETE CASCADE
) STRICT;

CREATE TABLE tag_table (
    tag_id INTEGER PRIMARY KEY,
    tag_name TEXT NOT NULL,
    tag_urgency REAL NOT NULL DEFAULT 0.0
) STRICT;

CREATE TABLE task_tag_table (
    task_id BLOB NOT NULL,
    tag_id INTEGER NOT NULL,

    PRIMARY KEY (task_id, tag_id),

    FOREIGN KEY (task_id) REFERENCES task_table (task_id),
    FOREIGN KEY (tag_id) REFERENCES tag_table (tag_id)
) STRICT;

-- Populate with default data

INSERT INTO status_table (status_id, status_name)
VALUES
(0, 'pending'),
(1, 'complete'),
(2, 'delete');

INSERT INTO priority_table (priority_id, priority_name, priority_urgency)
VALUES
(0, 'low', -3.0),
(1, 'medium', 3.0),
(2, 'high', 6.0);

-- SQL: down

DROP TABLE IF EXISTS uda_table;
DROP TABLE IF EXISTS task_tag_table;
DROP TABLE IF EXISTS tag_table;
DROP TABLE IF EXISTS task_table;
DROP TABLE IF EXISTS status_table;
DROP TABLE IF EXISTS priority_table;
DROP TABLE IF EXISTS project_table;

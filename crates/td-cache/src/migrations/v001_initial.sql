CREATE TABLE IF NOT EXISTS sync_state (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    section_id TEXT,
    parent_id TEXT,
    content TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    priority INTEGER NOT NULL DEFAULT 4,
    due_json TEXT,
    deadline_json TEXT,
    duration_json TEXT,
    labels_json TEXT NOT NULL DEFAULT '[]',
    "order" INTEGER,
    assignee_id TEXT,
    assigner_id TEXT,
    is_completed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT,
    updated_at TEXT,
    completed_at TEXT,
    url TEXT,
    raw_json TEXT
);

CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_tasks_section_id ON tasks(section_id);
CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_id);
CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority);
CREATE INDEX IF NOT EXISTS idx_tasks_is_completed ON tasks(is_completed);

CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    color TEXT,
    parent_id TEXT,
    "order" INTEGER,
    is_favorite INTEGER NOT NULL DEFAULT 0,
    is_inbox_project INTEGER NOT NULL DEFAULT 0,
    is_team_inbox INTEGER NOT NULL DEFAULT 0,
    view_style TEXT,
    url TEXT,
    raw_json TEXT
);

CREATE TABLE IF NOT EXISTS sections (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    "order" INTEGER,
    raw_json TEXT
);

CREATE INDEX IF NOT EXISTS idx_sections_project_id ON sections(project_id);

CREATE TABLE IF NOT EXISTS labels (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    color TEXT,
    "order" INTEGER,
    is_favorite INTEGER NOT NULL DEFAULT 0,
    raw_json TEXT
);

CREATE TABLE IF NOT EXISTS comments (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    project_id TEXT,
    content TEXT NOT NULL,
    posted_at TEXT,
    attachment_json TEXT,
    raw_json TEXT
);

CREATE INDEX IF NOT EXISTS idx_comments_task_id ON comments(task_id);
CREATE INDEX IF NOT EXISTS idx_comments_project_id ON comments(project_id);

CREATE TABLE IF NOT EXISTS collaborators (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    raw_json TEXT
);

CREATE TABLE IF NOT EXISTS reminders (
    id TEXT PRIMARY KEY,
    item_id TEXT NOT NULL,
    type TEXT,
    due_json TEXT,
    minute_offset INTEGER,
    raw_json TEXT
);

CREATE INDEX IF NOT EXISTS idx_reminders_item_id ON reminders(item_id);

CREATE TABLE IF NOT EXISTS filters (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    query TEXT NOT NULL,
    color TEXT,
    "order" INTEGER,
    is_favorite INTEGER NOT NULL DEFAULT 0,
    raw_json TEXT
);

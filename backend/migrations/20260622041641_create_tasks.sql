-- Add migration script here
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NULL,
    status INTEGER NOT NULL,
    priority INTEGER NOT NULL,
    due_date TEXT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
)

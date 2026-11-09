CREATE TABLE agent_runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    summary TEXT NOT NULL CHECK (json_valid(summary))
);

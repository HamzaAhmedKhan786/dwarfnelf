CREATE TABLE budgets (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    max_ram_gb REAL NOT NULL CHECK (max_ram_gb > 0),
    max_vram_gb REAL NOT NULL CHECK (max_vram_gb >= 0),
    max_ssd_gb REAL NOT NULL CHECK (max_ssd_gb >= 0),
    cpu_target_percent INTEGER NOT NULL CHECK (cpu_target_percent BETWEEN 1 AND 100)
);
INSERT INTO budgets VALUES (1, 1.0, 0.0, 1.0, 100);

CREATE VIRTUAL TABLE knowledge_chunks USING fts5(
    source UNINDEXED,
    chunk UNINDEXED,
    body,
    tokenize = 'unicode61'
);

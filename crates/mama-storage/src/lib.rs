use mama_api::Budget;
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::{path::Path, time::Duration};

pub fn open(path: &Path) -> rusqlite::Result<Connection> {
    let mut connection = Connection::open(path)?;
    connection.busy_timeout(Duration::from_secs(3))?;
    connection.pragma_update(None, "journal_mode", "WAL")?;
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let version = tx.pragma_query_value(None, "user_version", |row| row.get::<_, u32>(0))?;
    const MIGRATIONS: [&str; 2] = [
        include_str!("../../../migrations/0001_local_state.sql"),
        include_str!("../../../migrations/0002_agent_runs.sql"),
    ];
    if version as usize > MIGRATIONS.len() {
        return Err(rusqlite::Error::InvalidQuery);
    }
    for (index, migration) in MIGRATIONS.iter().enumerate().skip(version as usize) {
        tx.execute_batch(migration)?;
        tx.pragma_update(None, "user_version", index + 1)?;
    }
    tx.commit()?;
    Ok(connection)
}

pub fn read_budget(connection: &Connection) -> rusqlite::Result<Budget> {
    connection.query_row(
        "SELECT max_ram_gb, max_vram_gb, max_ssd_gb, cpu_target_percent FROM budgets WHERE id = 1",
        [],
        |row| {
            Ok(Budget {
                max_ram_gb: row.get(0)?,
                max_vram_gb: row.get(1)?,
                max_ssd_gb: row.get(2)?,
                cpu_target_percent: row.get(3)?,
            })
        },
    )
}

pub fn write_budget(connection: &Connection, budget: &Budget) -> rusqlite::Result<()> {
    connection.execute(
        "UPDATE budgets SET max_ram_gb=?1, max_vram_gb=?2, max_ssd_gb=?3, cpu_target_percent=?4 WHERE id=1",
        params![budget.max_ram_gb, budget.max_vram_gb, budget.max_ssd_gb, budget.cpu_target_percent],
    )?;
    Ok(())
}

pub fn save_run_summary(connection: &mut Connection, summary: &str) -> rusqlite::Result<i64> {
    let tx = connection.transaction()?;
    tx.execute("INSERT INTO agent_runs (summary) VALUES (?1)", [summary])?;
    let id = tx.last_insert_rowid();
    tx.execute(
        "DELETE FROM agent_runs WHERE id < (SELECT id FROM agent_runs ORDER BY id DESC LIMIT 1 OFFSET 99)",
        [],
    )?;
    tx.commit()?;
    Ok(id)
}

pub fn read_run_summary(
    connection: &Connection,
    id: i64,
) -> rusqlite::Result<Option<(String, String)>> {
    connection
        .query_row(
            "SELECT created_at, summary FROM agent_runs WHERE id = ?1",
            [id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs, process, time::SystemTime};

    #[test]
    fn saves_budget_across_reopen_and_rejects_newer_schema() {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = env::temp_dir().join(format!("dwarfnelf-storage-{}-{nonce}", process::id()));
        fs::create_dir(&dir).unwrap();
        let path = dir.join("state.sqlite");
        let budget = Budget {
            max_ram_gb: 3.0,
            ..Budget::default()
        };
        {
            let db = Connection::open(&path).unwrap();
            db.execute_batch(include_str!("../../../migrations/0001_local_state.sql"))
                .unwrap();
            db.pragma_update(None, "user_version", 1).unwrap();
            write_budget(&db, &budget).unwrap();
        }
        {
            let mut db = open(&path).unwrap();
            assert_eq!(read_budget(&db).unwrap(), budget);
            for id in 1..=101 {
                assert_eq!(save_run_summary(&mut db, "{}").unwrap(), id);
            }
        }
        {
            let db = open(&path).unwrap();
            assert!(read_run_summary(&db, 1).unwrap().is_none());
            assert_eq!(read_run_summary(&db, 2).unwrap().unwrap().1, "{}");
            db.pragma_update(None, "user_version", 3).unwrap();
        }
        assert!(open(&path).is_err());
        fs::remove_file(path).unwrap();
        fs::remove_dir(dir).unwrap();
    }
}

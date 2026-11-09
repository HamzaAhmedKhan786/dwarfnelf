use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Import {
    pub source: String,
    pub text: String,
}

impl Import {
    pub fn valid(&self) -> bool {
        !self.source.trim().is_empty()
            && self.source.len() <= 512
            && !self.source.chars().any(char::is_control)
            && !self.text.trim().is_empty()
            && self.text.len() <= 128 * 1024
            && !self.text.contains('\0')
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Search {
    pub query: String,
}

#[derive(Serialize)]
pub struct Hit {
    pub source: String,
    pub chunk: usize,
    pub text: String,
}

pub fn replace(connection: &mut Connection, input: &Import) -> rusqlite::Result<usize> {
    let tx = connection.transaction()?;
    tx.execute(
        "DELETE FROM knowledge_chunks WHERE source = ?1",
        [&input.source],
    )?;
    let words: Vec<_> = input.text.split_whitespace().collect();
    let mut count = 0;
    {
        let mut insert = tx.prepare("INSERT INTO knowledge_chunks VALUES (?1, ?2, ?3)")?;
        for (index, chunk) in words.chunks(200).enumerate() {
            insert.execute(params![input.source, index, chunk.join(" ")])?;
            count += 1;
        }
    }
    tx.commit()?;
    Ok(count)
}

pub fn search(connection: &Connection, query: &str) -> rusqlite::Result<Vec<Hit>> {
    let terms = query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|term| !term.is_empty())
        .take(24)
        .map(|term| format!("\"{term}\""))
        .collect::<Vec<_>>();
    if terms.is_empty() {
        return Ok(Vec::new());
    }
    let mut statement = connection.prepare(
        "SELECT source, chunk, body FROM knowledge_chunks
         WHERE knowledge_chunks MATCH ?1 ORDER BY bm25(knowledge_chunks), source, chunk LIMIT 10",
    )?;
    let rows = statement.query_map([terms.join(" OR ")], |row| {
        Ok(Hit {
            source: row.get(0)?,
            chunk: row.get(1)?,
            text: row.get(2)?,
        })
    })?;
    rows.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn indexes_replaces_and_cites_sources() {
        let mut db = mama_storage::open(Path::new(":memory:")).unwrap();
        let mut input = Import {
            source: "fixture".into(),
            text: "Rust memory safety".into(),
        };
        replace(&mut db, &input).unwrap();
        let hits = search(&db, "\"Rust\"*").unwrap();
        assert_eq!((hits[0].source.as_str(), hits[0].chunk), ("fixture", 0));
        input.text = "SQLite storage".into();
        replace(&mut db, &input).unwrap();
        assert!(search(&db, "rust").unwrap().is_empty());
        assert_eq!(search(&db, "sqlite").unwrap().len(), 1);
    }
}

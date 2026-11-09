use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize)]
#[serde(tag = "tool", rename_all = "snake_case", deny_unknown_fields)]
pub enum Step {
    KnowledgeSearch { query: String },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workflow {
    steps: Vec<Step>,
}

impl Workflow {
    pub fn valid(&self) -> bool {
        (1..=4).contains(&self.steps.len())
            && self.steps.iter().all(|step| match step {
                Step::KnowledgeSearch { query } => !query.trim().is_empty() && query.len() <= 1024,
            })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Completed,
    DeadlineReached,
    ToolFailed,
}

#[derive(PartialEq, Serialize)]
pub struct Citation {
    source: String,
    chunk: usize,
}

#[derive(Serialize)]
pub struct Trace {
    step: usize,
    tool: &'static str,
    hit_count: usize,
    elapsed_ms: u64,
}

#[derive(Serialize)]
pub struct Run {
    pub status: Status,
    pub abstained: bool,
    pub citations: Vec<Citation>,
    pub trace: Vec<Trace>,
}

pub fn run(db: &rusqlite::Connection, workflow: Workflow, deadline: Instant) -> Run {
    let mut result = Run {
        status: Status::Completed,
        abstained: true,
        citations: Vec::new(),
        trace: Vec::new(),
    };
    for (step, action) in workflow.steps.into_iter().enumerate() {
        let start = Instant::now();
        if start >= deadline {
            result.status = Status::DeadlineReached;
            break;
        }
        let Step::KnowledgeSearch { query } = action;
        let hits = mama_knowledge::search(db, &query);
        result.trace.push(Trace {
            step,
            tool: "knowledge_search",
            hit_count: hits.as_ref().map_or(0, Vec::len),
            elapsed_ms: start.elapsed().as_millis() as u64,
        });
        if Instant::now() >= deadline {
            result.status = Status::DeadlineReached;
            break;
        }
        match hits {
            Ok(hits) => {
                for hit in hits {
                    let citation = Citation {
                        source: hit.source,
                        chunk: hit.chunk,
                    };
                    if !result.citations.contains(&citation) {
                        result.citations.push(citation);
                    }
                }
            }
            Err(_) => {
                result.status = Status::ToolFailed;
                break;
            }
        }
    }
    result.abstained = result.citations.is_empty();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expired_workflow_never_invokes_its_tool() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        let workflow = Workflow {
            steps: vec![Step::KnowledgeSearch {
                query: "fixture".into(),
            }],
        };
        let result = run(&db, workflow, Instant::now());
        assert!(matches!(result.status, Status::DeadlineReached));
        assert!(result.trace.is_empty());
        assert!(result.abstained);
    }
}

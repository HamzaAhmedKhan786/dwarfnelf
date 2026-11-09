use axum::{
    extract::{rejection::JsonRejection, DefaultBodyLimit, Path, Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use mama_api::{Budget, ErrorBody};
use mama_knowledge::{Import, Search};
use mama_security::LocalAccess;
use rusqlite::Connection;
use serde_json::json;
use std::{
    env, io,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

#[derive(Clone)]
struct AppState {
    access: Arc<LocalAccess>,
    db: Arc<Mutex<Connection>>,
}

type ApiError = (StatusCode, Json<ErrorBody>);

fn error(status: StatusCode, code: &'static str, message: &'static str) -> ApiError {
    (
        status,
        Json(ErrorBody {
            code,
            error: message,
        }),
    )
}

fn payload<T>(body: Result<Json<T>, JsonRejection>) -> Result<T, ApiError> {
    body.map(|Json(value)| value).map_err(|rejection| {
        error(
            rejection.status(),
            "invalid_json",
            "Expected a valid JSON request body",
        )
    })
}

async fn database<T: Send + 'static>(
    state: AppState,
    operation: impl FnOnce(&mut Connection) -> rusqlite::Result<T> + Send + 'static,
) -> Result<T, ApiError> {
    tokio::task::spawn_blocking(move || {
        let mut connection = state.db.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        operation(&mut connection)
    })
    .await
    .ok()
    .and_then(Result::ok)
    .ok_or_else(|| {
        error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "storage_error",
            "Local storage operation failed",
        )
    })
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { Json(json!({"status": "ok", "version": env!("CARGO_PKG_VERSION")})) }))
        .route("/v1/capabilities", get(|| async { Json(json!({
            "api_version": "v1",
            "features": ["health", "capabilities", "budgets", "structured_errors", "hardware", "knowledge_lexical", "cpu_admission", "agent_read_only"]
        })) }))
        .route("/v1/hardware", get(hardware))
        .route("/v1/budgets", get(read_budget).put(update_budget))
        .route("/v1/knowledge/import", post(import_text))
        .route("/v1/knowledge/search", post(search_text))
        .route("/v1/decisions/admission", post(plan_admission))
        .route("/v1/agents/runs", post(run_workflow))
        .route("/v1/agents/runs/:id", get(read_workflow_summary))
        .fallback(|| async { error(StatusCode::NOT_FOUND, "not_found", "Route not found") })
        .method_not_allowed_fallback(|| async {
            error(StatusCode::METHOD_NOT_ALLOWED, "method_not_allowed", "Method not allowed")
        })
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .layer(middleware::from_fn_with_state(state.clone(), guard))
        .with_state(state)
}

fn single_header(headers: &HeaderMap, key: header::HeaderName) -> Option<&str> {
    let values = headers.get_all(key);
    let mut entries = values.iter();
    let value = entries.next()?.to_str().ok()?;
    entries.next().is_none().then_some(value)
}

async fn guard(State(state): State<AppState>, request: Request, next: Next) -> Response {
    static REQUESTS: AtomicU64 = AtomicU64::new(1);
    let request_id = REQUESTS.fetch_add(1, Ordering::Relaxed).to_string();
    let start = Instant::now();
    let headers = request.headers();
    let mut response = if request.uri().authority().is_some()
        || !state.access.allows_host(
            single_header(headers, header::HOST),
            headers.contains_key(header::ORIGIN) || headers.contains_key("sec-fetch-site"),
        ) {
        error(
            StatusCode::FORBIDDEN,
            "local_access_only",
            "Use the numeric loopback address from a native client",
        )
        .into_response()
    } else if !(request.uri().path() == "/health"
        || state
            .access
            .authenticates(single_header(headers, header::AUTHORIZATION)))
    {
        let mut response = error(
            StatusCode::UNAUTHORIZED,
            "authentication_required",
            "A valid local bearer token is required",
        )
        .into_response();
        response.headers_mut().insert(
            header::WWW_AUTHENTICATE,
            header::HeaderValue::from_static("Bearer"),
        );
        response
    } else {
        next.run(request).await
    };
    response.headers_mut().insert(
        "x-request-id",
        request_id.parse().expect("numeric request ID"),
    );
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store"),
    );
    tracing::info!(%request_id, status = response.status().as_u16(), elapsed_ms = start.elapsed().as_millis() as u64, "request");
    response
}

async fn read_budget(State(state): State<AppState>) -> Result<Json<Budget>, ApiError> {
    database(state, |db| mama_storage::read_budget(db))
        .await
        .map(Json)
}

async fn update_budget(
    State(state): State<AppState>,
    body: Result<Json<Budget>, JsonRejection>,
) -> Result<Json<Budget>, ApiError> {
    let budget = payload(body)?;
    budget
        .validate()
        .map_err(|message| error(StatusCode::BAD_REQUEST, "invalid_budget", message))?;
    database(state, move |db| {
        mama_storage::write_budget(db, &budget)?;
        Ok(budget)
    })
    .await
    .map(Json)
}

async fn hardware() -> Result<Json<mama_hardware::Hardware>, ApiError> {
    tokio::task::spawn_blocking(mama_hardware::inspect)
        .await
        .map(Json)
        .map_err(|_| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "hardware_error",
                "Hardware inspection failed",
            )
        })
}

async fn plan_admission(
    State(state): State<AppState>,
    body: Result<Json<mama_scheduler::Estimate>, JsonRejection>,
) -> Result<Json<mama_scheduler::Admission>, ApiError> {
    let required = payload(body)?.total().ok_or_else(|| {
        error(
            StatusCode::BAD_REQUEST,
            "invalid_estimate",
            "Provide nonzero memory estimates whose sum fits in u64",
        )
    })?;
    let (Json(budget), Json(snapshot)) = tokio::try_join!(read_budget(State(state)), hardware())?;
    Ok(Json(mama_scheduler::plan_cpu(
        required,
        &budget,
        snapshot.total_ram_bytes,
        snapshot.available_ram_bytes,
        snapshot.logical_cpus,
    )))
}

async fn run_workflow(
    State(state): State<AppState>,
    body: Result<Json<mama_agents::Workflow>, JsonRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let workflow = payload(body)?;
    if !workflow.valid() {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "invalid_workflow",
            "Provide one to four nonempty knowledge searches of up to 1024 UTF-8 bytes each",
        ));
    }
    let deadline = Instant::now() + Duration::from_secs(2);
    database(state, move |db| {
        let run = mama_agents::run(db, workflow, deadline);
        let summary = json!({"status": run.status, "abstained": run.abstained, "trace": run.trace});
        let id = mama_storage::save_run_summary(db, &summary.to_string())?;
        Ok(json!({"id": id, "status": run.status, "abstained": run.abstained, "trace": run.trace, "citations": run.citations}))
    }).await.map(Json)
}

async fn read_workflow_summary(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let id = id.parse::<i64>().ok().filter(|id| *id > 0).ok_or_else(|| {
        error(
            StatusCode::BAD_REQUEST,
            "invalid_run_id",
            "Provide a positive run ID",
        )
    })?;
    let (created_at, summary) = database(state, move |db| mama_storage::read_run_summary(db, id))
        .await?
        .ok_or_else(|| {
            error(
                StatusCode::NOT_FOUND,
                "run_not_found",
                "Run summary is unavailable",
            )
        })?;
    let summary: serde_json::Value = serde_json::from_str(&summary).map_err(|_| {
        error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "storage_error",
            "Local run summary is invalid",
        )
    })?;
    Ok(Json(
        json!({"id": id, "created_at": created_at, "summary": summary}),
    ))
}

async fn import_text(
    State(state): State<AppState>,
    body: Result<Json<Import>, JsonRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let input = payload(body)?;
    if !input.valid() {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "invalid_document",
            "Provide a source label and up to 128 KiB of nonempty UTF-8 text",
        ));
    }
    let chunks = database(state, move |db| mama_knowledge::replace(db, &input)).await?;
    Ok(Json(json!({"chunks": chunks})))
}

async fn search_text(
    State(state): State<AppState>,
    body: Result<Json<Search>, JsonRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let input = payload(body)?;
    if input.query.trim().is_empty() || input.query.len() > 1024 {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "invalid_query",
            "Provide a nonempty query of up to 1024 UTF-8 bytes",
        ));
    }
    let hits = database(state, move |db| mama_knowledge::search(db, &input.query)).await?;
    Ok(Json(json!({"abstained": hits.is_empty(), "hits": hits})))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .json()
        .init();
    let port = env::var("DWARFNELF_PORT").map_or(Ok(8765), |value| value.parse::<u16>())?;
    let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).await?;
    let addr = listener.local_addr()?;
    let token = env::var("DWARFNELF_TOKEN")
        .map_err(|_| io::Error::other("Set DWARFNELF_TOKEN before starting the daemon"))?;
    let access = LocalAccess::new(addr.port(), &token).map_err(io::Error::other)?;
    let path = env::var_os("DWARFNELF_DB")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .ok_or_else(|| {
            io::Error::other("Set DWARFNELF_DB to an absolute path in a private directory")
        })?;
    let db = mama_storage::open(&path)?;
    tracing::info!(%addr, "dwarfnelf daemon listening");
    axum::serve(
        listener,
        router(AppState {
            access: Arc::new(access),
            db: Arc::new(Mutex::new(db)),
        }),
    )
    .with_graceful_shutdown(async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(%error, "failed to listen for shutdown signal");
        }
    })
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn workflow_searches_and_rejects_unregistered_tools() {
        let token = "a".repeat(64);
        let mut db = mama_storage::open(std::path::Path::new(":memory:")).unwrap();
        mama_knowledge::replace(
            &mut db,
            &Import {
                source: "fixture".into(),
                text: "Rust ownership".into(),
            },
        )
        .unwrap();
        let app = router(AppState {
            access: Arc::new(LocalAccess::new(8765, &token).unwrap()),
            db: Arc::new(Mutex::new(db)),
        });
        let request = |route, body| {
            Request::builder()
                .method("POST")
                .uri(route)
                .header(header::HOST, "127.0.0.1:8765")
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap()
        };
        let response = app.clone().oneshot(request("/v1/agents/runs", r#"{"steps":[{"tool":"knowledge_search","query":"Rust"},{"tool":"knowledge_search","query":"ownership"}]}"#)).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096)
            .await
            .unwrap();
        let run: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(run["status"], "completed");
        assert_eq!(run["abstained"], false);
        assert_eq!(run["citations"], json!([{"source":"fixture","chunk":0}]));
        assert_eq!(run["trace"].as_array().unwrap().len(), 2);
        let history = Request::builder()
            .uri("/v1/agents/runs/1")
            .header(header::HOST, "127.0.0.1:8765")
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(history).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096)
            .await
            .unwrap();
        let saved: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(saved["id"], run["id"]);
        assert_eq!(saved["summary"]["trace"], run["trace"]);
        assert!(saved["summary"].get("citations").is_none());
        assert!(!String::from_utf8_lossy(&bytes).contains("ownership"));
        let response = app
            .clone()
            .oneshot(request(
                "/v1/agents/runs",
                r#"{"steps":[{"tool":"shell","command":"whoami"}]}"#,
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let response = app
            .oneshot(request(
                "/v1/decisions/admission",
                r#"{"weights_bytes":1000000001,"kv_cache_bytes":1,"runtime_bytes":1}"#,
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4096)
            .await
            .unwrap();
        let decision: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decision["outcome"], "exceeds_configured_budget");
    }

    #[tokio::test]
    async fn guards_routes_and_preserves_budget_contract() {
        let token = "a".repeat(64);
        let app = router(AppState {
            access: Arc::new(LocalAccess::new(8765, &token).unwrap()),
            db: Arc::new(Mutex::new(
                mama_storage::open(std::path::Path::new(":memory:")).unwrap(),
            )),
        });
        let auth = format!("Bearer {token}");
        for (host, origin, credential, expected) in [
            ("127.0.0.1:8765", None, None, StatusCode::UNAUTHORIZED),
            (
                "rebind.example:8765",
                None,
                Some(auth.as_str()),
                StatusCode::FORBIDDEN,
            ),
            (
                "127.0.0.1:8765",
                Some("https://attacker.example"),
                Some(auth.as_str()),
                StatusCode::FORBIDDEN,
            ),
            ("127.0.0.1:8765", None, Some(auth.as_str()), StatusCode::OK),
        ] {
            let mut request = Request::builder()
                .uri("/v1/budgets")
                .header(header::HOST, host);
            if let Some(origin) = origin {
                request = request.header(header::ORIGIN, origin);
            }
            if let Some(credential) = credential {
                request = request.header(header::AUTHORIZATION, credential);
            }
            let response = app
                .clone()
                .oneshot(request.body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), expected);
            assert!(response.headers().contains_key("x-request-id"));
        }
        let request = Request::builder()
            .method("PUT")
            .uri("/v1/budgets")
            .header(header::HOST, "127.0.0.1:8765")
            .header(header::AUTHORIZATION, &auth)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"max_ram_gb":0,"max_ssd_gb":1}"#))
            .unwrap();
        assert_eq!(
            app.oneshot(request).await.unwrap().status(),
            StatusCode::BAD_REQUEST
        );
    }
}

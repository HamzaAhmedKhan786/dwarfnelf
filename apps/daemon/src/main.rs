use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    budget: Arc<RwLock<Budget>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Budget {
    max_ram_gb: f64,
    #[serde(default)]
    max_vram_gb: f64,
    max_ssd_gb: f64,
    #[serde(default = "default_cpu_target")]
    cpu_target_percent: u8,
}

fn default_cpu_target() -> u8 {
    100
}

impl Budget {
    fn validate(&self) -> Result<(), String> {
        if !self.max_ram_gb.is_finite() || self.max_ram_gb <= 0.0 {
            return Err("max_ram_gb must be greater than zero".into());
        }
        if !self.max_vram_gb.is_finite() || self.max_vram_gb < 0.0 {
            return Err("max_vram_gb must not be negative".into());
        }
        if !self.max_ssd_gb.is_finite() || self.max_ssd_gb < 0.0 {
            return Err("max_ssd_gb must not be negative".into());
        }
        if !(1..=100).contains(&self.cpu_target_percent) {
            return Err("cpu_target_percent must be between 1 and 100".into());
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct Capabilities {
    api_version: &'static str,
    features: [&'static str; 4],
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .json()
        .init();
    let state = AppState {
        budget: Arc::new(RwLock::new(Budget {
            max_ram_gb: 1.0,
            max_vram_gb: 0.0,
            max_ssd_gb: 1.0,
            cpu_target_percent: 100,
        })),
    };
    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/capabilities", get(capabilities))
        .route("/v1/budgets", put(update_budget).get(read_budget))
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    let addr: SocketAddr = "127.0.0.1:8765".parse().expect("valid listen address");
    tracing::info!(%addr, "dwarfnelf daemon listening");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind daemon port");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("serve daemon");
}

async fn shutdown() {
    let _ = tokio::signal::ctrl_c().await;
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn capabilities() -> Json<Capabilities> {
    Json(Capabilities {
        api_version: "v1",
        features: ["health", "capabilities", "budgets", "structured_errors"],
    })
}

async fn read_budget(State(state): State<AppState>) -> Json<Budget> {
    Json(state.budget.read().await.clone())
}

async fn update_budget(
    State(state): State<AppState>,
    Json(budget): Json<Budget>,
) -> Result<Json<Budget>, (StatusCode, Json<ErrorBody>)> {
    budget
        .validate()
        .map_err(|error| (StatusCode::BAD_REQUEST, Json(ErrorBody { error })))?;
    *state.budget.write().await = budget.clone();
    Ok(Json(budget))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_budget() -> Budget {
        Budget {
            max_ram_gb: 4.0,
            max_vram_gb: 2.0,
            max_ssd_gb: 20.0,
            cpu_target_percent: 50,
        }
    }

    #[test]
    fn accepts_valid_budget() {
        assert!(valid_budget().validate().is_ok());
    }

    #[test]
    fn rejects_zero_ram() {
        let mut budget = valid_budget();
        budget.max_ram_gb = 0.0;
        assert_eq!(
            budget.validate().unwrap_err(),
            "max_ram_gb must be greater than zero"
        );
    }

    #[test]
    fn rejects_cpu_target_outside_range() {
        let mut budget = valid_budget();
        budget.cpu_target_percent = 0;
        assert_eq!(
            budget.validate().unwrap_err(),
            "cpu_target_percent must be between 1 and 100"
        );
    }
}

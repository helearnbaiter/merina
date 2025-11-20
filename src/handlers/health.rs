use crate::utils::{success_response, AppResult};
use axum::{response::Json, routing::get, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: i64,
}

pub async fn health_check() -> AppResult<Json<serde_json::Value>> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    Ok(Json(serde_json::json!(response)))
}

pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
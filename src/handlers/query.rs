use crate::datafusion_adapters::query_engine::{QueryEngine, QueryRequest, QueryResult};
use crate::utils::{success_response, AppResult};
use axum::{
    extract::{Json, State},
    response::Json as AxumJson,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ExecuteQueryRequest {
    pub sql: String,
}

#[derive(Serialize)]
pub struct ExecuteQueryResponse {
    pub schema: String,
    pub rows: Vec<serde_json::Value>,
    pub execution_time_ms: u64,
    pub row_count: usize,
}

pub async fn execute_query(
    State(query_engine): State<Arc<QueryEngine>>,
    Json(request): Json<ExecuteQueryRequest>,
) -> AppResult<AxumJson<ExecuteQueryResponse>> {
    let query_request = QueryRequest {
        sql: request.sql,
        data_source_ids: vec![], // For now, we're not requiring specific data sources
        limit: None,
    };

    let result = query_engine.execute_query(query_request).await?;
    
    Ok(AxumJson(ExecuteQueryResponse {
        schema: result.schema,
        rows: result.rows,
        execution_time_ms: result.execution_time_ms,
        row_count: result.row_count,
    }))
}

pub fn query_routes() -> Router {
    Router::new().route("/api/query/execute", post(execute_query))
}
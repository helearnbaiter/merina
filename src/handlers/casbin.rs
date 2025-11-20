use crate::services::casbin_service::CasbinService;
use crate::utils::{success_response, AppResult};
use axum::{
    extract::{Json, Path, Query, State},
    response::Json as AxumJson,
    routing::{delete, get, post, put},
    Router,
};
use casbin::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PolicyRequest {
    pub sub: String,
    pub obj: String,
    pub act: String,
}

#[derive(Serialize, Deserialize)]
pub struct CasbinRuleRequest {
    pub ptype: String,
    pub v0: Option<String>,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}

#[derive(Serialize)]
pub struct CasbinRuleResponse {
    pub id: Option<i32>,
    pub ptype: String,
    pub v0: Option<String>,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}

pub async fn add_policy(
    State(casbin_service): State<Arc<CasbinService>>,
    Json(request): Json<PolicyRequest>,
) -> AppResult<AxumJson<serde_json::Value>> {
    let result = casbin_service
        .add_policy(&request.sub, &request.obj, &request.act)
        .await?;

    Ok(AxumJson(serde_json::json!({
        "success": result,
        "message": "Policy added successfully"
    })))
}

pub async fn remove_policy(
    State(casbin_service): State<Arc<CasbinService>>,
    Json(request): Json<PolicyRequest>,
) -> AppResult<AxumJson<serde_json::Value>> {
    let result = casbin_service
        .remove_policy(&request.sub, &request.obj, &request.act)
        .await?;

    Ok(AxumJson(serde_json::json!({
        "success": result,
        "message": "Policy removed successfully"
    })))
}

pub async fn get_policies(
    State(casbin_service): State<Arc<CasbinService>>,
) -> AppResult<AxumJson<serde_json::Value>> {
    let policies = casbin_service.get_all_policies().await?;
    Ok(AxumJson(serde_json::json!(policies)))
}

pub async fn add_casbin_rule(
    State(casbin_service): State<Arc<CasbinService>>,
    Json(request): Json<CasbinRuleRequest>,
) -> AppResult<AxumJson<CasbinRuleResponse>> {
    let rule = casbin_service
        .add_rule_to_db(
            request.ptype,
            request.v0,
            request.v1,
            request.v2,
            request.v3,
            request.v4,
            request.v5,
        )
        .await?;

    Ok(AxumJson(CasbinRuleResponse {
        id: rule.id,
        ptype: rule.ptype,
        v0: rule.v0,
        v1: rule.v1,
        v2: rule.v2,
        v3: rule.v3,
        v4: rule.v4,
        v5: rule.v5,
    }))
}

pub async fn get_casbin_rules(
    State(casbin_service): State<Arc<CasbinService>>,
) -> AppResult<AxumJson<Vec<CasbinRuleResponse>>> {
    let rules = casbin_service.get_all_rules_from_db().await?;
    let response_rules: Vec<CasbinRuleResponse> = rules
        .into_iter()
        .map(|rule| CasbinRuleResponse {
            id: rule.id,
            ptype: rule.ptype,
            v0: rule.v0,
            v1: rule.v1,
            v2: rule.v2,
            v3: rule.v3,
            v4: rule.v4,
            v5: rule.v5,
        })
        .collect();

    Ok(AxumJson(response_rules))
}

pub async fn get_casbin_rule_by_id(
    State(casbin_service): State<Arc<CasbinService>>,
    Path(id): Path<i32>,
) -> AppResult<AxumJson<CasbinRuleResponse>> {
    // For simplicity, we'll return an error since we can't directly fetch by ID from the DB
    // In a real implementation, you'd have a method to get a single rule by ID
    Err(crate::utils::AppError::InternalError(
        "Method not implemented".to_string(),
    ))
}

pub async fn update_casbin_rule(
    State(casbin_service): State<Arc<CasbinService>>,
    Path(id): Path<i32>,
    Json(request): Json<CasbinRuleRequest>,
) -> AppResult<AxumJson<CasbinRuleResponse>> {
    let rule = casbin_service
        .update_rule_in_db(
            id,
            Some(request.ptype),
            request.v0,
            request.v1,
            request.v2,
            request.v3,
            request.v4,
            request.v5,
        )
        .await?;

    Ok(AxumJson(CasbinRuleResponse {
        id: rule.id,
        ptype: rule.ptype,
        v0: rule.v0,
        v1: rule.v1,
        v2: rule.v2,
        v3: rule.v3,
        v4: rule.v4,
        v5: rule.v5,
    }))
}

pub async fn delete_casbin_rule(
    State(casbin_service): State<Arc<CasbinService>>,
    Path(id): Path<i32>,
) -> AppResult<AxumJson<serde_json::Value>> {
    let rows_affected = casbin_service.delete_rule_from_db(id).await?;
    Ok(AxumJson(serde_json::json!({
        "success": rows_affected > 0,
        "deleted_count": rows_affected,
        "message": "Rule deleted successfully"
    })))
}

pub fn casbin_routes() -> Router {
    Router::new()
        .route("/api/casbin/policies", post(add_policy))
        .route("/api/casbin/policies", delete(remove_policy))
        .route("/api/casbin/policies", get(get_policies))
        .route("/api/casbin/rules", post(add_casbin_rule))
        .route("/api/casbin/rules", get(get_casbin_rules))
        .route("/api/casbin/rules/:id", get(get_casbin_rule_by_id))
        .route("/api/casbin/rules/:id", put(update_casbin_rule))
        .route("/api/casbin/rules/:id", delete(delete_casbin_rule))
}
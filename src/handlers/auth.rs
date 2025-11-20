use crate::utils::auth::{generate_jwt_token, Claims};
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
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

pub async fn login(
    State(config): State<Arc<crate::config::Config>>,
    Json(request): Json<LoginRequest>,
) -> AppResult<AxumJson<LoginResponse>> {
    // In a real implementation, you would validate the user credentials against a database
    // For now, we'll just create a token for any user
    let claims = Claims::new(
        request.username,
        vec!["user".to_string()], 
        vec!["read".to_string(), "write".to_string()]
    );
    
    let access_token = generate_jwt_token(&claims, &config.jwt.secret)?;
    let refresh_token = crate::utils::auth::generate_refresh_token(&claims.sub, &config.jwt.secret)?;
    
    let response = LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: config.jwt.expiration,
    };

    Ok(AxumJson(response))
}

pub async fn refresh_token(
    State(config): State<Arc<crate::config::Config>>,
    Json(_request): Json<serde_json::Value>, // In a real app, you'd have a refresh token request struct
) -> AppResult<AxumJson<LoginResponse>> {
    // In a real implementation, you would validate the refresh token
    // For now, we'll just return a new access token
    let claims = Claims::new(
        "refreshed_user".to_string(),
        vec!["user".to_string()], 
        vec!["read".to_string(), "write".to_string()]
    );
    
    let access_token = generate_jwt_token(&claims, &config.jwt.secret)?;
    let refresh_token = crate::utils::auth::generate_refresh_token(&claims.sub, &config.jwt.secret)?;
    
    let response = LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: config.jwt.expiration,
    };

    Ok(AxumJson(response))
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh_token))
}
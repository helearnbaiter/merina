use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Casbin error: {0}")]
    CasbinError(#[from] casbin::Error),
    
    #[error("DataFusion error: {0}")]
    DataFusionError(#[from] datafusion::error::DataFusionError),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Authorization error: {0}")]
    AuthzError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

// Error response structure
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            AppError::JwtError(err) => {
                tracing::error!("JWT error: {}", err);
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
            AppError::CasbinError(err) => {
                tracing::error!("Casbin error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Authorization system error")
            }
            AppError::DataFusionError(err) => {
                tracing::error!("DataFusion error: {}", err);
                (StatusCode::BAD_REQUEST, "Query processing error")
            }
            AppError::AuthError(msg) => {
                tracing::error!("Auth error: {}", msg);
                (StatusCode::UNAUTHORIZED, &msg)
            }
            AppError::AuthzError(msg) => {
                tracing::error!("Authz error: {}", msg);
                (StatusCode::FORBIDDEN, &msg)
            }
            AppError::ConfigError(err) => {
                tracing::error!("Config error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
            AppError::ValidationError(msg) => {
                tracing::error!("Validation error: {}", msg);
                (StatusCode::BAD_REQUEST, &msg)
            }
            AppError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, &msg)
            }
        };

        let body = Json(ErrorResponse {
            error: status.canonical_reason().unwrap_or("Unknown error").to_string(),
            message: error_message.to_string(),
            status_code: status.as_u16(),
        });

        (status, body).into_response()
    }
}

// Success response structure
#[derive(serde::Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
    pub message: Option<String>,
    pub status_code: u16,
}

pub fn success_response<T>(data: T, message: Option<String>) -> SuccessResponse<T> {
    SuccessResponse {
        data,
        message,
        status_code: 200,
    }
}

// Result alias for convenience
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response() {
        let error = AppError::AuthError("Invalid credentials".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
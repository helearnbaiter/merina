use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    ConfigError(config::ConfigError),
    ValidationError(String),
    Unauthorized,
    Forbidden,
    NotFound(String),
    InternalServerError(String),
    CasbinError(casbin::Error),
    JwtError(jsonwebtoken::errors::Error),
    OAuth2Error(oauth2::RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>),
    DataFusionError(datafusion::error::DataFusionError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::ConfigError(e) => write!(f, "Config error: {}", e),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden => write!(f, "Forbidden"),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::CasbinError(e) => write!(f, "Casbin error: {}", e),
            AppError::JwtError(e) => write!(f, "JWT error: {}", e),
            AppError::OAuth2Error(e) => write!(f, "OAuth2 error: {}", e),
            AppError::DataFusionError(e) => write!(f, "DataFusion error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err)
    }
}

impl From<casbin::Error> for AppError {
    fn from(err: casbin::Error) -> Self {
        AppError::CasbinError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError(err)
    }
}

impl From<oauth2::RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>> for AppError {
    fn from(err: oauth2::RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>) -> Self {
        AppError::OAuth2Error(err)
    }
}

impl From<datafusion::error::DataFusionError> for AppError {
    fn from(err: datafusion::error::DataFusionError) -> Self {
        AppError::DataFusionError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, &msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "Not found"),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, &msg),
            AppError::CasbinError(_) => (StatusCode::FORBIDDEN, "Authorization error"),
            AppError::JwtError(_) => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AppError::OAuth2Error(_) => (StatusCode::UNAUTHORIZED, "OAuth2 error"),
            AppError::DataFusionError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DataFusion error"),
        };

        let body = Json(json!({
            "error": error_message,
            "status_code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
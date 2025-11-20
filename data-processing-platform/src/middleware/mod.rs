use axum::{
    extract::{Extension, Request, State},
    http::{header::HeaderName, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::{errors::AppError, AppState};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract token from header
    let token = request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    // If no token is provided, allow public routes
    if token.is_none() {
        // For now, let's allow all routes without authentication
        // In a real application, you'd want to check against a list of public routes
        return Ok(next.run(request).await);
    }

    // Here you would validate the JWT token
    // For now, just pass through to the next middleware
    Ok(next.run(request).await)
}

pub async fn cors_middleware(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let mut response = next.run(request).await;
    
    // Add CORS headers
    response.headers_mut().insert(
        HeaderName::from_static("access-control-allow-origin"),
        HeaderValue::from_static("*"),
    );
    response.headers_mut().insert(
        HeaderName::from_static("access-control-allow-headers"),
        HeaderValue::from_static("*"),
    );
    response.headers_mut().insert(
        HeaderName::from_static("access-control-allow-methods"),
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );

    Ok(response)
}
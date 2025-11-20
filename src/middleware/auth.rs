use crate::services::casbin_service::CasbinService;
use crate::utils::auth::validate_jwt_token;
use crate::utils::{AppError, AppResult};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn auth_middleware(
    State(casbin_service): State<Arc<CasbinService>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    match auth_header {
        Some(token) => {
            // Validate JWT token
            let claims = match validate_jwt_token(token, &get_jwt_secret()) {
                Ok(claims) => claims,
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            };

            // Check if the user is authorized for this resource
            let path = request.uri().path();
            let method = request.method().as_str();
            
            // For now, we'll use a simple mapping - in production this should be more sophisticated
            let resource = map_path_to_resource(path);
            let action = map_method_to_action(method);
            
            match casbin_service.enforce(&claims.sub, &resource, &action).await {
                Ok(true) => {
                    // Add user info to request extensions
                    let mut request = request;
                    request.extensions_mut().insert(claims);
                    
                    Ok(next.run(request).await)
                }
                Ok(false) => Err(StatusCode::FORBIDDEN),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        None => {
            // For public endpoints, allow the request to continue
            // In a real implementation, you might have a list of public routes
            if is_public_route(request.uri().path()) {
                Ok(next.run(request).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string())
}

fn map_path_to_resource(path: &str) -> String {
    // Simple mapping of paths to resources
    // In a real implementation, you'd want more sophisticated path matching
    path.to_string()
}

fn map_method_to_action(method: &str) -> String {
    match method {
        "GET" => "read".to_string(),
        "POST" => "create".to_string(),
        "PUT" | "PATCH" => "update".to_string(),
        "DELETE" => "delete".to_string(),
        _ => "other".to_string(),
    }
}

fn is_public_route(path: &str) -> bool {
    // List of public routes that don't require authentication
    matches!(path, "/" | "/health" | "/api/auth/login" | "/api/auth/register")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_public_route() {
        assert!(is_public_route("/"));
        assert!(is_public_route("/health"));
        assert!(!is_public_route("/api/users"));
    }
}
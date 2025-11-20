use axum::{routing::get, routing::post, Router};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/auth/login", post(crate::handlers::auth::login))
        .route("/auth/refresh", post(crate::handlers::auth::refresh_token))
        .route("/auth/logout", post(crate::handlers::auth::logout))
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(crate::handlers::user::get_users))
        .route("/users", post(crate::handlers::user::create_user))
        .route("/users/:id", get(crate::handlers::user::get_user))
        .route("/users/:id", put(crate::handlers::user::update_user))
        .route("/users/:id", delete(crate::handlers::user::delete_user))
}

pub fn permission_routes() -> Router {
    Router::new()
        .route("/permissions", get(crate::handlers::permission::get_permissions))
        .route("/permissions", post(crate::handlers::permission::create_permission))
        .route("/permissions/:id", delete(crate::handlers::permission::delete_permission))
        .route("/permissions/check", post(crate::handlers::permission::check_permission))
}

pub fn datafusion_routes() -> Router {
    Router::new()
        .route("/datafusion/query", post(crate::handlers::datafusion::execute_query))
        .route("/datafusion/datasources", get(crate::handlers::datafusion::get_data_sources))
        .route("/datafusion/datasources", post(crate::handlers::datafusion::create_data_source))
        .route("/datafusion/datasources/:id", get(crate::handlers::datafusion::get_data_source))
        .route("/datafusion/datasources/:id", put(crate::handlers::datafusion::update_data_source))
        .route("/datafusion/datasources/:id", delete(crate::handlers::datafusion::delete_data_source))
}

use axum::routing::put;
use axum::routing::delete;
//! Modern Data Processing Platform
//! 
//! A comprehensive data processing platform built with Rust, featuring:
//! - Axum web framework
//! - Casbin for authorization
//! - OAuth2 with JWT
//! - PostgreSQL database with connection pooling
//! - DataFusion for analytics
//! - Flight SQL for distributed queries
//! - ADBC for database connectivity

mod config;
mod database;
mod errors;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use axum::{
    extract::State,
    http::Method,
    response::Json,
    routing::{get, post},
    Router,
};
use config::AppConfig;
use database::Database;
use errors::AppError;
use serde_json::json;
use std::sync::Arc;
use tokio;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load configuration
    let config = AppConfig::new()?;
    info!("Configuration loaded");

    // Initialize database
    let db = Database::new(&config.database).await?;
    info!("Database connected");

    // Initialize Casbin
    let enforcer = services::casbin_service::initialize_enforcer(&config.casbin.model_path).await?;
    info!("Casbin enforcer initialized");

    // Create shared state
    let app_state = Arc::new(AppState {
        config: config.clone(),
        db,
        enforcer,
    });

    // Build the application with routes
    let app = Router::new()
        // Health check
        .route("/health", get(health_handler))
        // Include all routes
        .merge(routes::auth_routes())
        .merge(routes::user_routes())
        .merge(routes::permission_routes())
        .merge(routes::datafusion_routes())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(Any),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.server.address).await?;
    info!("Server starting on {}", config.server.address);
    
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: Database,
    pub enforcer: casbin::Enforcer,
}

async fn health_handler() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({
        "status": "ok",
        "message": "Data Processing Platform is running"
    })))
}

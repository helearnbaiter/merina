use axum::{middleware, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod datafusion_adapters;
mod handlers;
mod middleware;
mod services;
mod utils;

use config::Config;
use datafusion_adapters::{DataSourceManager, QueryEngine};
use handlers::{auth_routes, casbin_routes, data_source_routes, health_routes, query_routes};
use middleware::{auth::auth_middleware, cors::cors_layer};
use services::casbin_service::CasbinService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "modern_data_platform=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting server with config: {:?}", config);

    // Initialize database connection pool
    let pool = sqlx::PgPool::connect(&config.database.url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Initialize Casbin service
    let casbin_service = Arc::new(
        CasbinService::new(pool.clone(), &config.casbin.model_conf_path)
            .await
            .expect("Failed to initialize Casbin service")
    );

    // Initialize DataFusion components
    let data_source_manager = Arc::new(DataSourceManager::new());
    let query_engine = Arc::new(QueryEngine::new());

    // Initialize Flight SQL server if enabled
    if config.datafusion.enable_flight_server {
        let flight_server = datafusion_adapters::FlightSqlServer::new();
        tokio::spawn(async move {
            if let Err(e) = flight_server.start_server(config.datafusion.flight_port).await {
                tracing::error!("Flight server error: {}", e);
            }
        });
        tracing::info!("Flight SQL server started on port {}", config.datafusion.flight_port);
    }

    // Build the application with routes
    let app = Router::new()
        // Health check route (public)
        .merge(health_routes())
        // Auth routes (public)
        .merge(auth_routes())
        // Protected routes with auth middleware
        .merge(casbin_routes())
        .merge(data_source_routes())
        .merge(query_routes())
        // Add middleware
        .layer(cors_layer())
        .layer(middleware::from_fn_with_state(
            casbin_service.clone(),
            auth_middleware,
        ))
        // Add state
        .with_state(AppState {
            config: Arc::new(config),
            casbin_service,
            data_source_manager,
            query_engine,
        });

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080)); // Default port, can be changed via config
    tracing::info!("Server running on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub casbin_service: Arc<CasbinService>,
    pub data_source_manager: Arc<DataSourceManager>,
    pub query_engine: Arc<QueryEngine>,
}
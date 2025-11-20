use crate::datafusion_adapters::data_source::{DataSourceConfig, DataSourceManager, DataSourceType};
use crate::utils::{success_response, AppResult};
use axum::{
    extract::{Json, Path, State},
    response::Json as AxumJson,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateDataSourceRequest {
    pub name: String,
    pub r#type: DataSourceType,
    pub connection_string: String,
    pub options: std::collections::HashMap<String, String>,
}

#[derive(Serialize)]
pub struct DataSourceResponse {
    pub id: String,
    pub name: String,
    pub r#type: DataSourceType,
    pub connection_string: String,
    pub options: std::collections::HashMap<String, String>,
    pub schema: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<DataSourceConfig> for DataSourceResponse {
    fn from(config: DataSourceConfig) -> Self {
        DataSourceResponse {
            id: config.id,
            name: config.name,
            r#type: config.r#type,
            connection_string: config.connection_string,
            options: config.options,
            schema: config.schema,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}

pub async fn create_data_source(
    State(data_source_manager): State<Arc<DataSourceManager>>,
    Json(request): Json<CreateDataSourceRequest>,
) -> AppResult<AxumJson<DataSourceResponse>> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();
    
    let config = DataSourceConfig {
        id: id.clone(),
        name: request.name,
        r#type: request.r#type,
        connection_string: request.connection_string,
        options: request.options,
        schema: None,
        created_at: now,
        updated_at: now,
    };

    data_source_manager.add_data_source(config.clone()).await?;
    
    // Register the data source with the query engine
    data_source_manager.register_data_source(&id).await?;

    Ok(AxumJson(config.into()))
}

pub async fn get_data_source(
    State(data_source_manager): State<Arc<DataSourceManager>>,
    Path(id): Path<String>,
) -> AppResult<AxumJson<DataSourceResponse>> {
    let config = data_source_manager.get_data_source(&id).await?;
    Ok(AxumJson(config.into()))
}

pub async fn update_data_source(
    State(data_source_manager): State<Arc<DataSourceManager>>,
    Path(id): Path<String>,
    Json(request): Json<CreateDataSourceRequest>,
) -> AppResult<AxumJson<DataSourceResponse>> {
    let now = chrono::Utc::now().naive_utc();
    let config = DataSourceConfig {
        id: id.clone(),
        name: request.name,
        r#type: request.r#type,
        connection_string: request.connection_string,
        options: request.options,
        schema: None,
        created_at: now, // Keep original creation time
        updated_at: now,
    };

    data_source_manager.update_data_source(id, config.clone()).await?;
    
    // Re-register the data source with the query engine
    data_source_manager.register_data_source(&config.id).await?;

    Ok(AxumJson(config.into()))
}

pub async fn delete_data_source(
    State(data_source_manager): State<Arc<DataSourceManager>>,
    Path(id): Path<String>,
) -> AppResult<AxumJson<serde_json::Value>> {
    data_source_manager.delete_data_source(&id).await?;
    Ok(AxumJson(serde_json::json!({
        "success": true,
        "message": "Data source deleted successfully"
    })))
}

pub async fn list_data_sources(
    State(data_source_manager): State<Arc<DataSourceManager>>,
) -> AppResult<AxumJson<Vec<DataSourceResponse>>> {
    let configs = data_source_manager.list_data_sources().await?;
    let responses: Vec<DataSourceResponse> = configs.into_iter().map(|c| c.into()).collect();
    Ok(AxumJson(responses))
}

pub fn data_source_routes() -> Router {
    Router::new()
        .route("/api/data-sources", post(create_data_source))
        .route("/api/data-sources", get(list_data_sources))
        .route("/api/data-sources/:id", get(get_data_source))
        .route("/api/data-sources/:id", put(update_data_source))
        .route("/api/data-sources/:id", delete(delete_data_source))
}
use crate::utils::{AppError, AppResult};
use arrow::datatypes::SchemaRef;
use datafusion::datasource::{TableProvider, TableType};
use datafusion::execution::context::SessionContext;
use datafusion::logical_expr::CreateExternalTable;
use datafusion::physical_plan::ExecutionPlan;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceType {
    Memory,
    CSV,
    PostgreSQL,
    MySQL,
    SQLite,
    Parquet,
    JSON,
    Arrow,
    Iceberg,
    Remote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceConfig {
    pub id: String,
    pub name: String,
    pub r#type: DataSourceType,
    pub connection_string: String,
    pub options: HashMap<String, String>,
    pub schema: Option<String>, // JSON representation of the schema
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct DataSourceManager {
    data_sources: Arc<RwLock<HashMap<String, DataSourceConfig>>>,
    ctx: Arc<RwLock<SessionContext>>,
}

impl DataSourceManager {
    pub fn new() -> Self {
        let ctx = SessionContext::new();
        DataSourceManager {
            data_sources: Arc::new(RwLock::new(HashMap::new())),
            ctx: Arc::new(RwLock::new(ctx)),
        }
    }

    pub async fn add_data_source(&self, config: DataSourceConfig) -> AppResult<()> {
        let mut data_sources = self.data_sources.write().await;
        data_sources.insert(config.id.clone(), config);
        Ok(())
    }

    pub async fn get_data_source(&self, id: &str) -> AppResult<DataSourceConfig> {
        let data_sources = self.data_sources.read().await;
        data_sources
            .get(id)
            .cloned()
            .ok_or_else(|| AppError::ValidationError(format!("Data source {} not found", id)))
    }

    pub async fn update_data_source(&self, id: String, config: DataSourceConfig) -> AppResult<()> {
        let mut data_sources = self.data_sources.write().await;
        if data_sources.contains_key(&id) {
            data_sources.insert(id, config);
            Ok(())
        } else {
            Err(AppError::ValidationError(format!(
                "Data source {} not found",
                id
            )))
        }
    }

    pub async fn delete_data_source(&self, id: &str) -> AppResult<()> {
        let mut data_sources = self.data_sources.write().await;
        data_sources.remove(id);
        Ok(())
    }

    pub async fn list_data_sources(&self) -> AppResult<Vec<DataSourceConfig>> {
        let data_sources = self.data_sources.read().await;
        Ok(data_sources.values().cloned().collect())
    }

    pub async fn register_data_source(&self, id: &str) -> AppResult<()> {
        let config = self.get_data_source(id).await?;
        let mut ctx = self.ctx.write().await;

        match config.r#type {
            DataSourceType::Memory => {
                // Memory tables are registered directly
                // This is a placeholder - actual implementation would depend on specific requirements
            }
            DataSourceType::CSV => {
                let table = datafusion::datasource::file_format::csv::CsvFormat::default()
                    .with_header(true)
                    .infer_schema(&ctx.state(), &config.connection_string)
                    .await?;

                let options = CreateExternalTable {
                    table_name: config.name.clone().into(),
                    location: config.connection_string.clone(),
                    file_type: datafusion::sql::parser::FileType::CSV,
                    has_header: true,
                    delimiter: b',',
                    schema: Arc::new(arrow::datatypes::Schema::empty()),
                    table_partition_cols: vec![],
                    if_not_exists: true,
                    file_compression_type: datafusion::datasource::file_format::file_type::FileCompressionType::UNCOMPRESSED,
                    options: Default::default(),
                    constraints: Default::default(),
                    column_defaults: Default::default(),
                };

                // Register the table
                ctx.register_table(&config.name, table.create_table_provider(&options).await?)?;
            }
            DataSourceType::Parquet => {
                // Register Parquet table
                ctx.register_parquet(&config.name, &config.connection_string, Default::default()).await?;
            }
            DataSourceType::PostgreSQL | DataSourceType::MySQL | DataSourceType::SQLite => {
                // For database sources, we'd use the ADBC adapter
                // Placeholder for ADBC implementation
                return Err(AppError::InternalError(
                    "Database data sources not fully implemented yet".to_string(),
                ));
            }
            DataSourceType::JSON => {
                // Register JSON table
                ctx.register_json(&config.name, &config.connection_string, Default::default()).await?;
            }
            DataSourceType::Arrow => {
                // Register Arrow table
                // Implementation would depend on specific Arrow file handling
                return Err(AppError::InternalError(
                    "Arrow data source not fully implemented yet".to_string(),
                ));
            }
            DataSourceType::Iceberg => {
                // Register Iceberg table
                // This would require Iceberg-specific implementation
                return Err(AppError::InternalError(
                    "Iceberg data source not fully implemented yet".to_string(),
                ));
            }
            DataSourceType::Remote => {
                // Handle remote data sources
                return Err(AppError::InternalError(
                    "Remote data source not fully implemented yet".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub async fn execute_query(&self, sql: &str) -> AppResult<impl futures::Stream<Item = Result<arrow::record_batch::RecordBatch, datafusion::error::DataFusionError>>> {
        let ctx = self.ctx.read().await;
        let df = ctx.sql(sql).await?;
        let plan = df.create_physical_plan().await?;
        let task_ctx = ctx.task_ctx();
        let stream = datafusion::physical_plan::execute_stream(plan, task_ctx)?;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_data_source_manager() {
        let manager = DataSourceManager::new();
        
        let config = DataSourceConfig {
            id: "test_source".to_string(),
            name: "Test Source".to_string(),
            r#type: DataSourceType::Memory,
            connection_string: "memory://test".to_string(),
            options: HashMap::new(),
            schema: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        manager.add_data_source(config.clone()).await.unwrap();
        let retrieved = manager.get_data_source("test_source").await.unwrap();
        
        assert_eq!(config.id, retrieved.id);
    }
}
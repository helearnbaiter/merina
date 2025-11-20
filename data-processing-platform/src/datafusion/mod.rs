use datafusion::prelude::*;
use datafusion::error::Result as DataFusionResult;
use std::sync::Arc;

pub struct DataFusionService {
    ctx: SessionContext,
}

impl DataFusionService {
    pub fn new() -> Self {
        let ctx = SessionContext::new();
        Self { ctx }
    }

    pub async fn execute_query(&self, sql: &str) -> DataFusionResult<ArrowJson> {
        let df = self.ctx.sql(sql).await?;
        let results = df.collect().await?;
        
        // Convert to JSON for API response
        let json_results = arrow::json::writer::record_batches_to_json_rows(&results)?;
        Ok(ArrowJson { data: json_results })
    }

    pub async fn register_csv(&self, name: &str, path: &str) -> DataFusionResult<()> {
        self.ctx.register_csv(name, path, CsvReadOptions::new()).await?;
        Ok(())
    }

    pub async fn register_postgres(&self, name: &str, connection_string: &str) -> DataFusionResult<()> {
        // For now, this is a placeholder - in a real implementation, you would connect to PostgreSQL
        // and register it as a table in DataFusion
        println!("Registering PostgreSQL table {} with connection: {}", name, connection_string);
        Ok(())
    }
}

#[derive(serde::Serialize)]
pub struct ArrowJson {
    pub data: Vec<arrow::json::writer::JsonRow>,
}
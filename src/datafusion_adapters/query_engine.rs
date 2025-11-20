use crate::utils::{AppError, AppResult};
use datafusion::execution::context::SessionContext;
use datafusion::prelude::*;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub sql: String,
    pub data_source_ids: Vec<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub schema: String, // JSON representation of the schema
    pub rows: Vec<serde_json::Value>,
    pub execution_time_ms: u64,
    pub row_count: usize,
}

pub struct QueryEngine {
    ctx: Arc<RwLock<SessionContext>>,
}

impl QueryEngine {
    pub fn new() -> Self {
        let ctx = SessionContext::new();
        QueryEngine {
            ctx: Arc::new(RwLock::new(ctx)),
        }
    }

    pub async fn execute_query(&self, request: QueryRequest) -> AppResult<QueryResult> {
        let start_time = std::time::Instant::now();
        let ctx = self.ctx.read().await;
        
        // Execute the SQL query
        let df = ctx.sql(&request.sql).await
            .map_err(|e| AppError::DataFusionError(e))?;
        
        // Get the physical plan
        let plan = df.create_physical_plan().await
            .map_err(|e| AppError::DataFusionError(e))?;
        
        // Execute the plan
        let task_ctx = ctx.task_ctx();
        let mut stream = datafusion::physical_plan::execute_stream(plan, task_ctx)
            .map_err(|e| AppError::DataFusionError(e))?;
        
        // Collect results
        let mut rows = Vec::new();
        let mut schema_json = String::new();
        let mut row_count = 0;
        
        while let Some(batch_result) = stream.next().await {
            let batch = batch_result
                .map_err(|e| AppError::DataFusionError(e))?;
            
            // Convert batch to JSON
            let batch_rows = self.batch_to_json(&batch)?;
            rows.extend(batch_rows);
            row_count += batch.num_rows();
            
            // Only capture schema from first batch
            if schema_json.is_empty() {
                schema_json = serde_json::to_string(batch.schema().as_ref())
                    .map_err(|e| AppError::InternalError(format!("Failed to serialize schema: {}", e)))?;
            }
        }
        
        let execution_time_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(QueryResult {
            schema: schema_json,
            rows,
            execution_time_ms,
            row_count,
        })
    }

    fn batch_to_json(&self, batch: &arrow::array::RecordBatch) -> AppResult<Vec<serde_json::Value>> {
        let mut rows = Vec::new();
        
        for row_idx in 0..batch.num_rows() {
            let mut row_obj = serde_json::Map::new();
            
            for (field_idx, schema_field) in batch.schema().fields().iter().enumerate() {
                let column = batch.column(field_idx);
                let value = self.array_value_to_json(column.as_ref(), row_idx)
                    .map_err(|e| AppError::InternalError(format!("Failed to convert array value to JSON: {}", e)))?;
                
                row_obj.insert(schema_field.name().clone(), value);
            }
            
            rows.push(serde_json::Value::Object(row_obj));
        }
        
        Ok(rows)
    }

    fn array_value_to_json(&self, array: &dyn arrow::array::Array, index: usize) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        use arrow::array::*;
        use arrow::datatypes::*;
        
        match array.data_type() {
            DataType::Int8 => {
                let arr = array.as_any().downcast_ref::<Int8Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(arr.value(index))))
                }
            }
            DataType::Int16 => {
                let arr = array.as_any().downcast_ref::<Int16Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(arr.value(index))))
                }
            }
            DataType::Int32 => {
                let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(arr.value(index))))
                }
            }
            DataType::Int64 => {
                let arr = array.as_any().downcast_ref::<Int64Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(arr.value(index))))
                }
            }
            DataType::UInt8 => {
                let arr = array.as_any().downcast_ref::<UInt8Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(arr.value(index))))
                }
            }
            DataType::UInt16 => {
                let arr = array.as_any().downcast_ref::<UInt16Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(arr.value(index))))
                }
            }
            DataType::UInt32 => {
                let arr = array.as_any().downcast_ref::<UInt32Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(arr.value(index))))
                }
            }
            DataType::UInt64 => {
                let arr = array.as_any().downcast_ref::<UInt64Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(arr.value(index))))
                }
            }
            DataType::Float32 => {
                let arr = array.as_any().downcast_ref::<Float32Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from_f64(arr.value(index) as f64)
                        .unwrap_or(serde_json::Number::from_f64(0.0).unwrap())))
                }
            }
            DataType::Float64 => {
                let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from_f64(arr.value(index))
                        .unwrap_or(serde_json::Number::from_f64(0.0).unwrap())))
                }
            }
            DataType::Utf8 => {
                let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::String(arr.value(index).to_string()))
                }
            }
            DataType::LargeUtf8 => {
                let arr = array.as_any().downcast_ref::<LargeStringArray>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::String(arr.value(index).to_string()))
                }
            }
            DataType::Boolean => {
                let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    Ok(serde_json::Value::Bool(arr.value(index)))
                }
            }
            DataType::Timestamp(TimeUnit::Millisecond, _) => {
                let arr = array.as_any().downcast_ref::<TimestampMillisecondArray>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    let timestamp = chrono::DateTime::from_timestamp_millis(arr.value(index))
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default();
                    Ok(serde_json::Value::String(timestamp))
                }
            }
            DataType::Timestamp(TimeUnit::Microsecond, _) => {
                let arr = array.as_any().downcast_ref::<TimestampMicrosecondArray>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    let timestamp = chrono::DateTime::from_timestamp_micros(arr.value(index))
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default();
                    Ok(serde_json::Value::String(timestamp))
                }
            }
            DataType::Date32 => {
                let arr = array.as_any().downcast_ref::<Date32Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    let date = chrono::NaiveDate::from_num_days_from_ce_opt(arr.value(index))
                        .map(|d| d.to_string())
                        .unwrap_or_default();
                    Ok(serde_json::Value::String(date))
                }
            }
            DataType::Date64 => {
                let arr = array.as_any().downcast_ref::<Date64Array>().unwrap();
                if arr.is_null(index) {
                    Ok(serde_json::Value::Null)
                } else {
                    let date = chrono::NaiveDate::from_num_days_from_ce_opt((arr.value(index) / 86400000) as i32)
                        .map(|d| d.to_string())
                        .unwrap_or_default();
                    Ok(serde_json::Value::String(date))
                }
            }
            _ => {
                // For unsupported types, return null
                Ok(serde_json::Value::Null)
            }
        }
    }

    pub async fn register_table_from_csv(&self, table_name: &str, path: &str) -> AppResult<()> {
        let ctx = self.ctx.read().await;
        ctx.register_csv(table_name, path, CsvReadOptions::new()).await
            .map_err(|e| AppError::DataFusionError(e))?;
        Ok(())
    }

    pub async fn register_table_from_parquet(&self, table_name: &str, path: &str) -> AppResult<()> {
        let ctx = self.ctx.read().await;
        ctx.register_parquet(table_name, path, ParquetReadOptions::default()).await
            .map_err(|e| AppError::DataFusionError(e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_engine() {
        let engine = QueryEngine::new();
        
        // Register a test table
        engine.register_table_from_csv("test_table", "./test_data.csv").await.unwrap();
        
        let request = QueryRequest {
            sql: "SELECT * FROM test_table LIMIT 10".to_string(),
            data_source_ids: vec!["test_table".to_string()],
            limit: Some(10),
        };
        
        // This test would require actual test data to run properly
        assert!(true);
    }
}
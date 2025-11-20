use crate::utils::{AppError, AppResult};
use arrow::ipc::writer::{DictionaryTracker, IpcDataGenerator};
use arrow::record_batch::RecordBatch;
use arrow_flight::encode::FlightDataEncoderBuilder;
use arrow_flight::error::FlightError;
use arrow_flight::flight_descriptor::DescriptorType;
use arrow_flight::flight_service_server::{FlightService, FlightServiceServer};
use arrow_flight::{
    Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
    HandshakeRequest, HandshakeResponse, PutResult, SchemaAsIpc, Ticket,
};
use datafusion::execution::context::SessionContext;
use futures::Stream;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status, Streaming};

pub struct FlightSqlServer {
    ctx: Arc<RwLock<SessionContext>>,
    tasks: Arc<RwLock<HashMap<String, String>>>, // task_id -> SQL query
}

impl FlightSqlServer {
    pub fn new() -> Self {
        let ctx = SessionContext::new();
        FlightSqlServer {
            ctx: Arc::new(RwLock::new(ctx)),
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_server(&self, port: u16) -> AppResult<()> {
        let addr = format!("0.0.0.0:{}", port).parse()
            .map_err(|e| AppError::InternalError(format!("Failed to parse address: {}", e)))?;
        
        let service = FlightServiceServer::new(self.clone());
        
        tonic::transport::Server::builder()
            .add_service(service)
            .serve(addr)
            .await
            .map_err(|e| AppError::InternalError(format!("Flight server error: {}", e)))?;
            
        Ok(())
    }
}

#[tonic::async_trait]
impl FlightService for FlightSqlServer {
    type HandshakeStream = Pin<Box<dyn Stream<Item = Result<HandshakeResponse, Status>> + Send>>;
    type ListFlightsStream = Pin<Box<dyn Stream<Item = Result<FlightInfo, Status>> + Send>>;
    type DoGetStream = Pin<Box<dyn Stream<Item = Result<FlightData, Status>> + Send>>;
    type DoPutStream = Pin<Box<dyn Stream<Item = Result<PutResult, Status>> + Send>>;
    type DoActionStream = Pin<Box<dyn Stream<Item = Result<arrow_flight::Result, Status>> + Send>>;
    type ListActionsStream = Pin<Box<dyn Stream<Item = Result<ActionType, Status>> + Send>>;

    async fn handshake(
        &self,
        _request: Request<Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        // For simplicity, we'll implement a basic handshake
        // In a real implementation, you'd validate credentials here
        let response = HandshakeResponse {
            protocol_version: 0,
            payload: b"authenticated".to_vec(),
        };

        let stream = futures::stream::iter(vec![Ok(response)]);
        Ok(Response::new(Box::pin(stream)))
    }

    async fn list_flights(
        &self,
        _request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        // Return an empty stream for now
        let stream = futures::stream::empty();
        Ok(Response::new(Box::pin(stream)))
    }

    async fn get_flight_info(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        let descriptor = request.into_inner();
        
        if descriptor.r#type != DescriptorType::Path as i32 {
            return Err(Status::invalid_argument("Expected path descriptor"));
        }

        let path = descriptor.path.join(".");
        if path.is_empty() {
            return Err(Status::invalid_argument("Path cannot be empty"));
        }

        // Execute the query to get schema
        let ctx = self.ctx.read().await;
        let df = ctx
            .sql(&path)
            .await
            .map_err(|e| Status::internal(format!("SQL execution error: {}", e)))?;
        
        let schema = df.schema().into();
        
        // Create FlightInfo
        let info = FlightInfo {
            schema: arrow::ipc::writer::IpcMessage::default().0,
            flight_descriptor: Some(descriptor.clone()),
            endpoint: vec![],
            total_records: 0,
            total_bytes: 0,
            ordered: false,
        };

        Ok(Response::new(info))
    }

    async fn do_get(
        &self,
        request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        let ticket = request.into_inner();
        let sql = String::from_utf8(ticket.ticket)
            .map_err(|_| Status::invalid_argument("Invalid ticket"))?;

        // Execute the query
        let ctx = self.ctx.read().await;
        let df = ctx
            .sql(&sql)
            .await
            .map_err(|e| Status::internal(format!("SQL execution error: {}", e)))?;
        
        let plan = df
            .create_physical_plan()
            .await
            .map_err(|e| Status::internal(format!("Physical plan creation error: {}", e)))?;
        
        let task_ctx = ctx.task_ctx();
        let stream = datafusion::physical_plan::execute_stream(plan, task_ctx)
            .map_err(|e| Status::internal(format!("Execution stream error: {}", e)))?;

        // Convert RecordBatch stream to FlightData stream
        let schema = df.schema().into();
        let schema_bytes = SchemaAsIpc::new(&schema, &Default::default())
            .try_into()
            .map_err(|e| Status::internal(format!("Schema serialization error: {}", e)))?;

        // Create a stream that sends the schema first, then the data
        let flight_data_stream = FlightDataEncoderBuilder::new()
            .build(stream)
            .map_err(|e| Status::internal(format!("Flight data encoding error: {}", e)))?;

        Ok(Response::new(Box::pin(flight_data_stream)))
    }

    async fn do_put(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        // For now, we don't support data uploads
        Err(Status::unimplemented("DoPut not implemented"))
    }

    async fn list_actions(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        // Return available actions
        let actions = vec![
            ActionType {
                r#type: "CreatePreparedStatement".to_string(),
                description: "Create a prepared statement".to_string(),
            },
            ActionType {
                r#type: "ExecutePreparedStatement".to_string(),
                description: "Execute a prepared statement".to_string(),
            },
            ActionType {
                r#type: "ClosePreparedStatement".to_string(),
                description: "Close a prepared statement".to_string(),
            },
        ];

        let stream = futures::stream::iter(actions.into_iter().map(Ok));
        Ok(Response::new(Box::pin(stream)))
    }

    async fn do_action(
        &self,
        request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        let action = request.into_inner();
        
        match action.r#type.as_str() {
            "CreatePreparedStatement" => {
                // Parse the action body to extract SQL
                let sql = String::from_utf8(action.body)
                    .map_err(|_| Status::invalid_argument("Invalid action body"))?;
                
                // Generate a task ID and store the SQL
                let task_id = uuid::Uuid::new_v4().to_string();
                {
                    let mut tasks = self.tasks.write().await;
                    tasks.insert(task_id.clone(), sql);
                }
                
                // Return the task ID as the result
                let result = arrow_flight::Result {
                    body: task_id.into_bytes(),
                };
                
                let stream = futures::stream::iter(vec![Ok(result)]);
                Ok(Response::new(Box::pin(stream)))
            }
            "ExecutePreparedStatement" => {
                // Extract task ID from the action body
                let task_id = String::from_utf8(action.body)
                    .map_err(|_| Status::invalid_argument("Invalid action body"))?;
                
                // Retrieve the SQL for the task
                let sql = {
                    let tasks = self.tasks.read().await;
                    tasks.get(&task_id)
                        .cloned()
                        .ok_or_else(|| Status::not_found("Task not found"))?
                };
                
                // Execute the query
                let ctx = self.ctx.read().await;
                let df = ctx
                    .sql(&sql)
                    .await
                    .map_err(|e| Status::internal(format!("SQL execution error: {}", e)))?;
                
                let plan = df
                    .create_physical_plan()
                    .await
                    .map_err(|e| Status::internal(format!("Physical plan creation error: {}", e)))?;
                
                let task_ctx = ctx.task_ctx();
                let stream = datafusion::physical_plan::execute_stream(plan, task_ctx)
                    .map_err(|e| Status::internal(format!("Execution stream error: {}", e)))?;

                // Convert to FlightData and return results
                let flight_data_stream = FlightDataEncoderBuilder::new()
                    .build(stream)
                    .map_err(|e| Status::internal(format!("Flight data encoding error: {}", e)))?;

                let result_stream = flight_data_stream
                    .map(|result| result.map_err(|e| Status::internal(format!("Flight data error: {}", e))))
                    .map(|result| result.map(|data| arrow_flight::Result { body: data.cmd }));
                
                Ok(Response::new(Box::pin(result_stream)))
            }
            "ClosePreparedStatement" => {
                // Remove the task from the store
                let task_id = String::from_utf8(action.body)
                    .map_err(|_| Status::invalid_argument("Invalid action body"))?;
                
                {
                    let mut tasks = self.tasks.write().await;
                    tasks.remove(&task_id);
                }
                
                let result = arrow_flight::Result {
                    body: b"closed".to_vec(),
                };
                
                let stream = futures::stream::iter(vec![Ok(result)]);
                Ok(Response::new(Box::pin(stream)))
            }
            _ => Err(Status::unimplemented(format!("Action {} not implemented", action.r#type))),
        }
    }
}

// Implement Clone for FlightSqlServer to work with tonic
impl Clone for FlightSqlServer {
    fn clone(&self) -> Self {
        FlightSqlServer {
            ctx: self.ctx.clone(),
            tasks: self.tasks.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_flight_server_creation() {
        let server = FlightSqlServer::new();
        assert!(server.ctx.read().await != std::sync::Arc::new(RwLock::new(SessionContext::new())));
    }
}
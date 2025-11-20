use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;
use tracing::info;

use crate::config::DatabaseConfig;

pub struct Database {
    pub pool: Arc<Pool<Postgres>>,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error> {
        info!("Connecting to database: {}", config.url);
        
        let pool = PgPool::connect_with(
            sqlx::postgres::PgConnectOptions::new()
                .connect_timeout(std::time::Duration::from_secs(config.connect_timeout))
                .idle_timeout(std::time::Duration::from_secs(config.idle_timeout))
                .max_connections(config.max_connections)
                .min_connections(config.min_connections)
                .from_str(&config.url)?,
        )
        .await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
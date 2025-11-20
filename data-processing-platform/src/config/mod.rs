use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub casbin: CasbinConfig,
    pub datafusion: DataFusionConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CasbinConfig {
    pub model_path: String,
    pub policy_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DataFusionConfig {
    pub max_memory: usize,
    pub temp_dir: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(Environment::with_prefix("APP"));

        // Allow overriding with environment variables
        s = s.add_source(Environment::default());

        s.build()?.try_deserialize()
    }
}
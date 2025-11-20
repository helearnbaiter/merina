use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub casbin: CasbinConfig,
    pub datafusion: DataFusionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub cors_origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64, // in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasbinConfig {
    pub model_conf_path: String,
    pub policy_table_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFusionConfig {
    pub enable_flight_server: bool,
    pub flight_port: u16,
    pub max_memory: usize,
    pub temp_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        
        let mut cfg = config::Config::builder()
            .add_source(config::File::with_name("config/base").required(false))
            .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"));

        // Set default values if not provided
        cfg = cfg
            .set_default("app.host", "0.0.0.0")?
            .set_default("app.port", 8080)?
            .set_default("app.cors_origin", "*")?
            .set_default("database.max_connections", 20)?
            .set_default("database.min_connections", 5)?
            .set_default("database.connect_timeout", 30)?
            .set_default("database.idle_timeout", 600)?
            .set_default("jwt.expiration", 3600)?
            .set_default("casbin.model_conf_path", "config/casbin/model.conf")?
            .set_default("casbin.policy_table_name", "casbin_rule")?
            .set_default("datafusion.enable_flight_server", false)?
            .set_default("datafusion.flight_port", 50051)?
            .set_default("datafusion.max_memory", 1073741824)? // 1GB
            .set_default("datafusion.temp_dir", "/tmp/datafusion")?;

        cfg.build()?.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        std::env::set_var("APP__DATABASE__URL", "postgres://test:test@localhost/test");
        std::env::set_var("APP__JWT__SECRET", "test_secret");
        
        let config = Config::from_env().unwrap();
        assert_eq!(config.database.url, "postgres://test:test@localhost/test");
        assert_eq!(config.jwt.secret, "test_secret");
    }
}
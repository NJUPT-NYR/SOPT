use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PgConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub dbname: String,
    pub max_connections: u32,
}

impl PgConfig {
    pub fn to_address(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.dbname)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub pg: PgConfig,
    pub redis: deadpool_redis::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
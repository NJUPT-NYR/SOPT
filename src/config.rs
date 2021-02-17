use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
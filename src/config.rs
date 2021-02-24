use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub redis: deadpool_redis::Config,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
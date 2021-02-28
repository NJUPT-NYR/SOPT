use config::ConfigError;
use serde::Deserialize;

/// Configs read from `.env` file
/// 1. server_addr: actix-web bind server
/// 2. redis
/// 3. database_url: postgres url, see
/// [Postgres Docs](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING)
/// for more information
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
use config::ConfigError;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::Arc;

lazy_static! {
    pub(crate) static ref CONFIG: Arc<Config> = Arc::new(Config::from_env().unwrap());
}

/// SMTP configuration when used to
/// send invitation codes
#[derive(Deserialize, Debug)]
pub(crate) struct SMTPAccount {
    pub server: String,
    pub username: String,
    pub password: String,
}

/// Configs read from `.env` file
/// 1. server_addr: actix-web bind server
/// 2. secret_key
/// 3. database_url: postgres url, see
/// [Postgres Docs](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING)
/// for more information
/// 4. tracker announce addr
/// 5. smtp configuration
/// 6. path for kv database, used to store
/// site configs
/// 7. path for s3/minio, used for object storage
#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub server_addr: String,
    pub tracker_addr: String,
    pub secret_key: String,
    pub database_url: String,
    pub announce_addr: String,
    pub smtp: SMTPAccount,
    pub kv_path: String,
    pub oss_path: String,
}

impl Config {
    fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

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

/// OSS configuration
#[derive(Deserialize, Debug)]
pub(crate) struct OSSConfig {
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

/// Configs read from `.env` file
/// 1. server_addr: actix-web bind server
/// 2. tracker_addr: actix-web binded for proxy(tracker)
/// 3. secret_key
/// 4. database_url: postgres url, see
/// [Postgres Docs](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING)
/// for more information
/// 5. tracker announce addr
/// 6. smtp configuration
#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub server_addr: String,
    pub tracker_addr: String,
    pub secret_key: String,
    pub database_url: String,
    pub announce_addr: String,
    pub smtp: SMTPAccount,
    pub oss: OSSConfig,
}

impl Config {
    fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
    }
}

pub mod client;

use client::Client;
use config::ConfigError;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::Arc;

lazy_static! {
    pub(crate) static ref CONFIG: Arc<Config> = Arc::new(Config::from_env().unwrap());
}

lazy_static! {
    pub static ref ALLOWED_CLIENT: Vec<Client> = vec![
        Client::UTorrent,
        Client::Vuze,
        Client::Transmission,
        Client::QBittorrent,
        Client::Aria,
    ];
}

pub fn default_num_want() -> u16 {
    50
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: String,
    pub tracker_addr: String,
    pub redis_uri: String,
    pub database_url: String,
}

impl Config {
    fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

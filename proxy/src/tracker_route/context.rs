use crate::config::client::Client;
use crate::config::{ALLOWED_CLIENT, CONFIG};
use crate::error::ProxyError;
use crate::filter::Filter;
use deadpool::managed;
use deadpool_redis::{redis::RedisError, Config, ConnectionWrapper};
use lazy_static::lazy_static;
use std::sync::Arc;

type Pool = managed::Pool<ConnectionWrapper, RedisError>;

lazy_static! {
    pub static ref CONTEXT: Arc<Context> = Arc::new(Context::new(&CONFIG.redis_uri));
}

pub struct Context {
    pub pool: Pool,
    pub filter: Filter,
    // TODO: monitor, LOGGER are needed
}

impl Context {
    pub fn new(uri: &str) -> Self {
        let mut cfg = Config::default();
        cfg.url = Some(uri.to_string());
        let pool = cfg.create_pool().expect("Create Redis Pool Failed!");
        let filter = Filter::new();
        Context { pool, filter }
    }

    pub async fn validation(
        &self,
        data: &super::data::AnnounceRequestData,
    ) -> Result<(), crate::error::ProxyError> {
        let client = Client::new(&data.peer_id)?;
        if !ALLOWED_CLIENT.contains(&client) {
            return Err(ProxyError::RequestError("Client not allowed!"));
        }

        if !self.filter.contains(&data.passkey).await {
            return Err(ProxyError::RequestError(
                "Passkey not found! Check your torrent please.",
            ));
        }
        Ok(())
    }
}

mod csv;
mod redis;
mod rocksdb;
#[cfg(feature = "sled")]
mod sled_embedded;

use crate::config::CONFIG;
use crate::error::Error;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref KVDB: Arc<dyn KVStorage> = Arc::new(
        #[cfg(feature = "sled")]
        {
            sled_embedded::SledWrapper {
                db: sled::open(&CONFIG.kv_path).expect("unable to open kv"),
            }
        }
    );
}

// TODO: structured value support is needed
pub trait KVStorage: Send + Sync {
    fn put(&self, cf: &str, key: &[u8], val: &[u8]) -> Result<(), Error>;
    fn get_string(&self, cf: &str, key: &[u8]) -> Result<Option<String>, Error>;
    fn get_number(&self, cf: &str, key: &[u8]) -> Result<Option<i64>, Error>;
    fn get_float(&self, cf: &str, key: &[u8]) -> Result<Option<f64>, Error>;
    fn delete(&self, cf: &str, key: &[u8]) -> Result<(), Error>;
}

mod csv_storage;
mod redis;
#[cfg(feature = "rocksdb")]
mod rocks;
#[cfg(feature = "sled")]
mod sled_embedded;

use crate::config::CONFIG;
use crate::error::Error;
use lazy_static::lazy_static;
use std::convert::TryInto;
use std::path::Path;
use std::sync::Arc;

cfg_if::cfg_if! {
    if #[cfg(feature = "sled")] {
        lazy_static! {
            pub static ref KVDB: Arc<dyn KVStorage> = Arc::new(sled_embedded::SledWrapper {
                db: sled::open(&CONFIG.kv_path).expect("unable to open kv"),
            });
        }
    } else if #[cfg(feature = "rocksdb")] {
        lazy_static! {
            pub static ref KVDB: Arc<dyn KVStorage> = Arc::new(rocks::RocksWrapper {
                db: rocksdb::DB::open_cf(
                    &rocksdb::Options::default(),
                    &CONFIG.kv_path,
                    &["config", "reset"],
                )
                .expect("unable to open kv"),
            });
        }
    } else {
        lazy_static! {
            pub static ref KVDB: Arc<dyn KVStorage> = Arc::new(csv_storage::CSVReader {
                path: String::from(&CONFIG.kv_path),
            });
        }
    }
}

// TODO: structured value support
pub trait KVStorage: Send + Sync {
    fn put(&self, cf: &str, key: &[u8], val: &[u8]) -> Result<(), Error>;
    fn get_string(&self, cf: &str, key: &[u8]) -> Result<Option<String>, Error>;
    fn get_number(&self, cf: &str, key: &[u8]) -> Result<Option<i64>, Error>;
    fn get_float(&self, cf: &str, key: &[u8]) -> Result<Option<f64>, Error>;
    fn delete(&self, cf: &str, key: &[u8]) -> Result<(), Error>;
}

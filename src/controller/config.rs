use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::RwLock;
use crate::config::CONFIG;

pub fn is_not_su(role: i64) -> bool {
    role & (1 << 63) == 0
}
pub fn is_no_permission_to_torrents(role: i64) -> bool {
    role & (1 << 62) == 0
}
pub fn is_no_permission_to_users(role: i64) -> bool {
    role & (1 << 61) == 0
}
pub fn is_no_permission_to_site(role: i64) -> bool {
    role & (1 << 60) == 0
}
pub fn cannot_invite(role: i64) -> bool {
    role & (1 << 1) == 0
}
pub fn is_not_ordinary_user(role: i64) -> bool {
    role & 1 == 0
}

lazy_static! {
    pub static ref ALLOWED_DOMAIN: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

lazy_static! {
    pub static ref ROCKSDB: rocksdb::DB = rocksdb::DB::open_default(&CONFIG.rocksdb_path)
        .expect("unable to connect to rocksdb");
}

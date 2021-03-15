use std::sync::atomic::AtomicI64;
use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::RwLock;

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

#[cfg(feature = "email-restriction")]
lazy_static! {
    pub(crate) static ref ALLOWED_DOMAIN: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

pub(crate) static SETTING_LIST_I64: [&str;1] = [
    "invite_consume",
];

pub(crate) static SETTING_LIST_BOOL: [&str;0] = [];

pub(crate) static INVITE_CONSUME: AtomicI64 = AtomicI64::new(5000);
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

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
pub fn cannot_send_msg(role: i64) -> bool {
    role & (1 << 2) == 0
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
    pub static ref STRING_SITE_SETTING: HashMap<&'static str, &'static str> = [
        ("SITE NAME", "SOPT"),
        ("ACTIVATE EMAIL", "Welcome to register SOPT!\n\nClick following address to activate: https://sopt.rs/auth/activate"),
        ("PASSWORD RESET EMAIL", "Code will be expired in 30 minutes.\n\nClick following address to reset your password: https://sopt.rs/auth/validate_reset"),
    ].iter().copied().collect();
}

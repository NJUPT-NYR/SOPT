use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn get_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() & (std::u64::MAX - 1)
}

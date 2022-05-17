use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_ms_in_unix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

// This function works like the get_ms_in_unix function, but returns seconds instead of milliseconds
pub fn get_sec_in_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

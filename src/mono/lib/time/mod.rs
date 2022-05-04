use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_ms_in_unix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

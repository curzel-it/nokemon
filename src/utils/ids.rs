use std::time::{SystemTime, UNIX_EPOCH};


pub fn get_next_id() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .saturating_sub(1720000000) as u32
}
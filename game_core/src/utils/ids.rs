use lazy_static::lazy_static;

use std::{sync::atomic::{AtomicU32, Ordering}, time::{SystemTime, UNIX_EPOCH}};

lazy_static! {
    pub static ref NEXT_ID: AtomicU32 = AtomicU32::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .saturating_sub(1720000000) as u32
    );
}

pub fn get_next_id() -> u32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
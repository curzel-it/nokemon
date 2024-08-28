use std::{sync::Mutex, time::{SystemTime, UNIX_EPOCH}};

lazy_static::lazy_static! {
    static ref COUNTER: Mutex<u32> = Mutex::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32
    );
}

pub fn get_next_id() -> u32 {
    let mut num = COUNTER.lock().unwrap();
    *num += 1;
    *num
}
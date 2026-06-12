use std::sync::atomic::{AtomicU64, Ordering};

pub static STATE_VERSION: AtomicU64 = AtomicU64::new(1);

pub fn get_state_version() -> u64 {
    STATE_VERSION.load(Ordering::SeqCst)
}

pub fn increment_state_version() {
    STATE_VERSION.fetch_add(1, Ordering::SeqCst);
}

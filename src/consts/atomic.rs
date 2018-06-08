
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};

pub static SHUTDOWN: AtomicBool = ATOMIC_BOOL_INIT;

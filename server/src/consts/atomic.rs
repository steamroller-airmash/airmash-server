use std::sync::atomic::{AtomicBool, AtomicUsize, ATOMIC_BOOL_INIT, ATOMIC_USIZE_INIT};

pub static SHUTDOWN: AtomicBool = ATOMIC_BOOL_INIT;

pub static NUM_PLAYERS: AtomicUsize = ATOMIC_USIZE_INIT;

use std::sync::atomic::{AtomicBool, AtomicUsize};

pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub static NUM_PLAYERS: AtomicUsize = AtomicUsize::new(0);

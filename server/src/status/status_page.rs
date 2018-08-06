use consts::NUM_PLAYERS;
use std::sync::atomic::Ordering;

pub fn generate_status_page() -> String {
	format!("{{\"players\":{}}}", NUM_PLAYERS.load(Ordering::Relaxed))
}

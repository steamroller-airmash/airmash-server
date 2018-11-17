
use std::time::Duration;

lazy_static! {
	/// The that must past before a missile
	/// ID can be reused. Missile IDs being
	/// reused causes the client to see
	/// ghost missiles.
	pub static ref ID_REUSE_TIME: Duration = Duration::from_secs(30);
}

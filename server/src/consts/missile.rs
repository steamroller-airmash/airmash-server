use std::time::Duration;

lazy_static! {
	/// The time that must pass before a missile
	/// ID can be reused. Missile IDs being
	/// reused while the client is still running
	/// animations for them causes the client to see
	/// ghost missiles.
	pub static ref ID_REUSE_TIME: Duration = Duration::from_secs(60 * 3);
}

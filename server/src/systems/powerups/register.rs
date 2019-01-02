use super::*;
use dispatch::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with::<CheckExpired>()
		.with::<SpawnShield>()
		.with_handler::<Pickup>()
		.with_handler::<SendDespawn>()
}

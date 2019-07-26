use super::*;
use crate::dispatch::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with_handler::<TriggerUpdate>()
		.with_handler::<SetPowerupLifetime>()
		.with_handler::<SendPlayerPowerup>()
}

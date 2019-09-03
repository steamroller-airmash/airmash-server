use crate::dispatch::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with_handler::<UnthrottlePlayer>()
		.with_handler::<DelayMessage>()
		.with_handler::<DeleteEntity>()
}

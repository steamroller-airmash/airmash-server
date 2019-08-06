use crate::dispatch::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with_handler::<Teleport>()
		.with_handler::<GivePowerup>()
		.with_handler::<Crash>()
		.with_handler::<DebugPrint>()
}

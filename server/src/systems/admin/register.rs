use dispatch::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with_handler::<SpawnUpgrade>()
		.with_handler::<Teleport>()
		.with_handler::<GivePowerup>()
}

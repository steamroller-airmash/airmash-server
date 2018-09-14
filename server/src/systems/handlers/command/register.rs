use dispatch::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with_handler::<Respawn>()
		.with_handler::<Spectate>()
		.with_handler::<Flag>()
}

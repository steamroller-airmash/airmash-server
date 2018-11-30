use dispatch::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with_handler::<PlayerRespawn>()
		.with::<UnthrottlePlayer>()
		.with::<LoginFailed>()
		.with::<LoginHandler>()
		.with::<DelayMessage>()
		.with_handler::<DeleteEntity>()
}

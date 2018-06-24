
use dispatch::Builder;
use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	let builder = game::register(builder);

	builder
}

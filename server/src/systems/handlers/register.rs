use super::*;
use dispatch::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with_registrar(game::register)
		.with_registrar(packet::register)
		.with_registrar(command::register)
}

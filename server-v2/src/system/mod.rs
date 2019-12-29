
pub mod packet;
pub mod builtin;

pub use self::inner::register;

mod inner {
	use super::*;
	use crate::ecs::Builder;

	pub fn register(builder: &mut Builder) {
		builder
			.with_registrar(builtin::register)
			.with_registrar(packet::register);
	}
}

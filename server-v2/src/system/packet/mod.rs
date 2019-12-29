
mod parse;
mod connect;
mod disconnect;

pub use self::parse::handle_message;
pub use self::connect::handle_connect;

use crate::ecs::Builder;

pub fn register(builder: &mut Builder) {
	builder.with::<handle_message>();
}

mod connect;
mod disconnect;
mod parse;

pub use self::connect::handle_connect;
pub use self::parse::handle_message;

use crate::ecs::Builder;

pub fn register(builder: &mut Builder) {
    builder.with::<handle_message>();
}

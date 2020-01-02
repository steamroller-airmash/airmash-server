mod connect;
mod disconnect;
mod key;
mod parse;

pub use self::connect::handle_connect;
pub use self::disconnect::handle_disconnect;
pub use self::parse::handle_message;

pub use self::key::handle_key;

use crate::ecs::Builder;

pub fn register(builder: &mut Builder) {
    builder
        .with::<handle_message>()
        .with::<handle_connect>()
        .with::<handle_disconnect>()
        // Per-packet handlers
        .with::<handle_key>();
}

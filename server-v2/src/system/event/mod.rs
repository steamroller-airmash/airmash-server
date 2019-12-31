//! Systems that handle the various events throughout
//! the server.

pub mod on_join;

pub use self::inner::register;

mod inner {
    use super::*;
    use crate::ecs::Builder;

    pub fn register(builder: &mut Builder) {
        builder.with_registrar(on_join::register);
    }
}

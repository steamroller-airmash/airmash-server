pub mod builtin;
pub mod event;
pub mod packet;

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

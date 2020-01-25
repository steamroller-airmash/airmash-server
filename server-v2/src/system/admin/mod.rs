//! Admin commands for use with debugging.
//!
//! These allow for directly modifying internal server
//! state and performing other debug actions.
//!
//! This includes stuff like
//! - Directly setting the position of an entity
//! - Shutting down the server
//! - More to come...
//!
//! Note however that none of these broadcast their changes
//! to other systems to this may result in things appearing
//! to be wrong. For example, teleporting the flags in CTF
//! doesn't necessarily end up sending the updated position
//! of said flags to client.

mod debug;
mod teleport;

pub use self::debug::dump_entity;
pub use self::inner::shutdown;
pub use self::teleport::*;

pub use self::registrar::register;

mod registrar {
    use super::teleport::teleport;
    use super::{dump_entity, shutdown};

    pub fn register(builder: &mut crate::ecs::Builder) {
        builder
            .with::<shutdown>()
            .with::<teleport>()
            .with::<dump_entity>();
    }
}

mod inner {
    use crate::ecs::prelude::*;
    use crate::protocol::client::Command;
    use crate::resource::{builtin::ShutdownFlag, packet::ClientPacket, Config};

    /// Admin command to shutdown the entire server.
    #[event_handler]
    pub fn shutdown<'a>(
        evt: &ClientPacket<Command<'static>>,

        config: &Read<'a, Config>,
        flag: &mut WriteExpect<'a, ShutdownFlag>,
    ) {
        if !config.admin_enabled {
            return;
        }

        if evt.packet.com != "shutdown" {
            return;
        }

        flag.shutdown();
    }
}

//! Admin commands for use with debugging

pub use self::inner::shutdown;

pub fn register(builder: &mut crate::ecs::Builder) {
    builder.with::<shutdown>();
}

mod inner {
    use crate::ecs::prelude::*;
    use crate::protocol::client::Command;
    use crate::resource::{builtin::ShutdownFlag, packet::ClientPacket, Config};

    #[event_handler]
    fn shutdown<'a>(
        evt: &ClientPacket<Command>,

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

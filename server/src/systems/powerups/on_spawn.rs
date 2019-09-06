//! Event handlers that run when a powerup spawns.

use crate::{
	component::powerup::*, protocol::server::MobUpdateStationary, types::systemdata::Connections,
};

#[event_handler(name=SendPacket)]
fn send_packet<'a>(evt: &PowerupSpawn, conns: &Connections<'a>) {
	conns.send_to_visible(
		evt.pos,
		MobUpdateStationary {
			id: evt.mob.into(),
			ty: evt.ty,
			pos: evt.pos,
		},
	);
}

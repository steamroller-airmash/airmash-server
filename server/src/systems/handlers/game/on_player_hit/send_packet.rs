use specs::prelude::*;

use crate::component::event::PlayerHit as PlayerHitEvt;
use crate::component::flag::*;
use crate::component::reference::PlayerRef;
use crate::types::systemdata::Connections;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::*;

use crate::protocol::server::PlayerHitPlayer;

#[derive(Default)]
pub struct SendPacket;

#[derive(SystemDataCustom)]
pub struct SendPacketData<'a> {
	config: Read<'a, Config>,
	conns: Connections<'a>,

	health: ReadStorage<'a, Health>,
	plane: ReadStorage<'a, Plane>,
	owner: ReadStorage<'a, PlayerRef>,

	mob: ReadStorage<'a, Mob>,
	pos: ReadStorage<'a, Position>,
	is_missile: ReadStorage<'a, IsMissile>,
}

impl EventHandlerTypeProvider for SendPacket {
	type Event = PlayerHitEvt;
}

impl<'a> EventHandler<'a> for SendPacket {
	type SystemData = SendPacketData<'a>;

	fn on_event(&mut self, evt: &PlayerHitEvt, data: &mut Self::SystemData) {
		if !data.is_missile.get(evt.missile).is_some() {
			return;
		}

		let pos = try_get!(evt.missile, data.pos);
		let mob = try_get!(evt.missile, data.mob);
		let owner = try_get!(evt.missile, data.owner);

		let health = try_get!(evt.player, data.health);
		let plane = try_get!(evt.player, data.plane);

		let ref planeconf = data.config.planes[*plane];

		let packet = crate::protocol::server::PlayerHit {
			id: evt.missile.into(),
			owner: owner.0.into(),
			pos: *pos,
			ty: *mob,
			players: vec![PlayerHitPlayer {
				id: evt.player.into(),
				health: *health,
				health_regen: planeconf.health_regen,
			}],
		};

		data.conns.send_to_visible(*pos, packet);
	}
}

system_info! {
	impl SystemInfo for SendPacket {
		type Dependencies = super::InflictDamage;
	}
}

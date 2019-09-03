use specs::prelude::*;

use crate::component::event::*;
use crate::component::flag::*;
use crate::systems::collision::PlayerMissileCollisionSystem;
use crate::types::systemdata::{Connections, IsAlive};
use crate::types::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

use crate::protocol::server::EventStealth;

#[derive(Default)]
pub struct DestealthOnHit;

#[derive(SystemData)]
pub struct DestealthOnHitData<'a> {
	conns: Connections<'a>,

	keystate: WriteStorage<'a, KeyState>,
	plane: ReadStorage<'a, Plane>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_alive: IsAlive<'a>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
}

impl EventHandlerTypeProvider for DestealthOnHit {
	type Event = PlayerMissileCollision;
}

impl<'a> EventHandler<'a> for DestealthOnHit {
	type SystemData = DestealthOnHitData<'a>;

	fn on_event(&mut self, evt: &PlayerMissileCollision, data: &mut Self::SystemData) {
		let &PlayerMissileCollision(evt) = evt;
		let player = data
			.is_player
			.get(evt.0.ent)
			.map(|_| evt.0.ent)
			.unwrap_or(evt.1.ent);

		if *try_get!(player, data.plane) != Plane::Prowler {
			return;
		}
		if !data.is_alive.get(player) {
			return;
		}

		try_get!(player, mut data.keystate).stealthed = false;

		let packet = EventStealth {
			id: player.into(),
			state: false,
			energy: *try_get!(player, data.energy),
			energy_regen: *try_get!(player, data.energy_regen),
		};

		data.conns.send_to_player(player, packet);
	}
}

system_info! {
	impl SystemInfo for DestealthOnHit {
		type Dependencies = PlayerMissileCollisionSystem;
	}
}

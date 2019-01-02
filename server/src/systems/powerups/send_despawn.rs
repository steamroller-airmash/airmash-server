use specs::*;

use types::collision::*;
use types::systemdata::*;

use component::event::*;
use component::flag::*;
use systems;
use utils::*;

use protocol::server::MobDespawn;
use protocol::DespawnType;

#[derive(Default)]
pub struct SendDespawn;

#[derive(SystemData)]
pub struct SendDespawnData<'a> {
	entities: Entities<'a>,
	conns: SendToVisible<'a>,

	is_player: ReadStorage<'a, IsPlayer>,
}

impl EventHandlerTypeProvider for SendDespawn {
	type Event = PlayerPowerupCollision;
}

impl<'a> EventHandler<'a> for SendDespawn {
	type SystemData = SendDespawnData<'a>;

	fn on_event(&mut self, evt: &PlayerPowerupCollision, data: &mut Self::SystemData) {
		let Collision(c1, c2) = evt.0;

		let (_, upgrade) = match data.is_player.get(c1.ent) {
			Some(_) => (c1, c2),
			None => (c2, c1),
		};

		if !data.entities.is_alive(upgrade.ent) {
			return;
		}

		data.conns.send_to_visible(
			upgrade.pos,
			MobDespawn {
				id: upgrade.ent.into(),
				ty: DespawnType::Collided
			}
		);
	}
}

system_info! {
	impl SystemInfo for SendDespawn {
		type Dependencies = systems::collision::PlayerPowerupCollisionSystem;
	}
}

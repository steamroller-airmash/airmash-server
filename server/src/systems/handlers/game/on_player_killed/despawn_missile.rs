use specs::*;
use types::*;

use SystemInfo;

use component::event::PlayerKilled;
use component::flag::IsDead;
use protocol::server::MobDespawnCoords;

use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct DespawnMissile;

#[derive(SystemData)]
pub struct DespawnMissileData<'a> {
	conns: Read<'a, Connections>,
	is_dead: WriteStorage<'a, IsDead>,
	mob: ReadStorage<'a, Mob>,
}

impl EventHandlerTypeProvider for DespawnMissile {
	type Event = PlayerKilled;
}

impl<'a> EventHandler<'a> for DespawnMissile {
	type SystemData = DespawnMissileData<'a>;

	fn on_event(&mut self, evt: &PlayerKilled, data: &mut Self::SystemData) {
		data.is_dead.insert(evt.player, IsDead).unwrap();

		let despawn_packet = MobDespawnCoords {
			id: evt.missile.into(),
			ty: *try_get!(evt.missile, data.mob),
			pos: evt.pos,
		};

		data.conns.send_to_visible(evt.pos, despawn_packet);
	}
}

impl SystemInfo for DespawnMissile {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

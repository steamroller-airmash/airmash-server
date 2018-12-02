use specs::*;

use server::component::flag::*;
use server::*;
use server::utils::*;

use systems::timer::GameStart;

/// Drops all players out of spec on
/// the game start.
///
/// This system is a companion to
/// [`RespawnAll`]. Since [`RespawnAll`]
/// uses the `PlayerRespawner` adapter
/// it cannot write to the storage of
/// `IsSpectating`, and thus that part
/// must be split out into a separate
/// system.
#[derive(Default)]
pub struct RespawnAllUnspec;

#[derive(SystemData)]
pub struct RespawnAllUnspecData<'a> {
	entities: Entities<'a>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_spec: WriteStorage<'a, IsSpectating>,
}

impl EventHandlerTypeProvider for RespawnAllUnspec {
	type Event = GameStart;
}

impl<'a> EventHandler<'a> for RespawnAllUnspec {
	type SystemData = RespawnAllUnspecData<'a>;

	fn on_event(&mut self, _: &GameStart, data: &mut Self::SystemData) {
		let ref mut is_spec = data.is_spec;

		(&*data.entities, data.is_player.mask())
			.join()
			.for_each(|(ent, ..)| {
				// Remove the spectating key if present,
				// otherwise leave it
				is_spec.remove(ent);
			});
	}
}

impl SystemInfo for RespawnAllUnspec {
	type Dependencies = GameStart;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

use specs::*;

use server::component::flag::*;
use server::*;

use component::*;
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
pub struct RespawnAllUnspec {
	reader: Option<OnGameStartReader>,
}

#[derive(SystemData)]
pub struct RespawnAllUnspecData<'a> {
	channel: Read<'a, OnGameStart>,

	entities: Entities<'a>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_spec: WriteStorage<'a, IsSpectating>,
}

impl<'a> System<'a> for RespawnAllUnspec {
	type SystemData = RespawnAllUnspecData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameStart>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let mut is_spec = data.is_spec;

		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			(&*data.entities, data.is_player.mask())
				.join()
				.for_each(|(ent, ..)| {
					// Remove the spectating key if present,
					// otherwise leave it
					is_spec.remove(ent);
				});
		}
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

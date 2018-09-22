use specs::*;

use component::channel::*;
use component::flag::*;
use types::*;
use SystemInfo;

use systems::handlers::command::AllCommandHandlers;
use systems::handlers::game::on_join::AllJoinHandlers;

/// Set transform, health, energy and flags
/// for a player when they respawn.
///
/// More specifically, this system removes the
/// [`IsDead`] flag and sets the following:
///
/// - [`Position`]
/// - [`Velocity`]
/// - [`Rotation`]
/// - [`Health`]
/// - [`Energy`]
#[derive(Default)]
pub struct SetTraits {
	reader: Option<OnPlayerRespawnReader>,
}

#[derive(SystemData)]
pub struct SetTraitsData<'a> {
	channel: Read<'a, OnPlayerRespawn>,

	team: ReadStorage<'a, Team>,
	pos: WriteStorage<'a, Position>,
	vel: WriteStorage<'a, Velocity>,
	rot: WriteStorage<'a, Rotation>,
	health: WriteStorage<'a, Health>,
	energy: WriteStorage<'a, Energy>,

	is_dead: WriteStorage<'a, IsDead>,

	gamemode: GameModeWriter<'a, GameMode>,
}

impl<'a> System<'a> for SetTraits {
	type SystemData = SetTraitsData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerRespawn>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let gamemode = data.gamemode.get_mut();

		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = evt.player;
			let team = *data.team.get(player).unwrap();
			let pos = gamemode.spawn_pos(player, team);

			*data.pos.get_mut(player).unwrap() = pos;
			*data.vel.get_mut(player).unwrap() = Velocity::default();
			*data.rot.get_mut(player).unwrap() = Rotation::default();
			*data.health.get_mut(player).unwrap() = Health::new(1.0);
			*data.energy.get_mut(player).unwrap() = Energy::new(1.0);

			data.is_dead.remove(player);
		}
	}
}

impl SystemInfo for SetTraits {
	type Dependencies = (
		AllJoinHandlers,
		AllCommandHandlers,
		super::CreateDespawnEvent,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

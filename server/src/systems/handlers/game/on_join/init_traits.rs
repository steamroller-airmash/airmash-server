
use specs::*;

use SystemInfo;
use GameMode;
use GameModeWriter;

use types::*;

use systems::handlers::packet::LoginHandler;

use protocol::{PlaneType, PlayerStatus};
use component::channel::*;

pub struct InitTraits {
	reader: Option<OnPlayerJoinReader>
}

#[derive(SystemData)]
pub struct InitTraitsData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub gamemode: GameModeWriter<'a, GameMode>,

	pub powerups: WriteStorage<'a, Powerups>,
	pub upgrades: WriteStorage<'a, Upgrades>,
	pub score: WriteStorage<'a, Score>,
	pub level: WriteStorage<'a, Level>,
	pub team: WriteStorage<'a, Team>,
	pub plane: WriteStorage<'a, Plane>,
	pub status: WriteStorage<'a, Status>,
}

impl<'a> System<'a> for InitTraits {
	type SystemData = InitTraitsData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerJoin>().register_reader()
		);
	}

	fn run(&mut self,	data: Self::SystemData) {
		let Self::SystemData {
			channel,
			mut gamemode,

			mut powerups,
			mut upgrades,
			mut score,
			mut level,
			mut team,
			mut plane,
			mut status,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let player_team = gamemode.get_mut().assign_team(evt.0);

			powerups.insert(evt.0, Powerups::default()).unwrap();
			upgrades.insert(evt.0, Upgrades::default()).unwrap();
			score.insert(evt.0, Score(0)).unwrap();
			level.insert(evt.0, Level(0)).unwrap();
			team.insert(evt.0, player_team).unwrap();
			plane.insert(evt.0, PlaneType::Predator).unwrap();
			status.insert(evt.0, PlayerStatus::Alive).unwrap();
		}
	}
}

impl SystemInfo for InitTraits {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self{ reader: None }
	}
}


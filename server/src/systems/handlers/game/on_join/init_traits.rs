use specs::*;

use SystemInfo;

use types::*;

use systems::handlers::packet::LoginHandler;

use component::channel::*;
use protocol::PlayerStatus;

pub struct InitTraits {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitTraitsData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,

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

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,

			mut score,
			mut level,
			mut team,
			mut plane,
			mut status,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			score.insert(evt.id, Score(0)).unwrap();
			level.insert(evt.id, evt.level).unwrap();
			team.insert(evt.id, evt.team).unwrap();
			plane.insert(evt.id, evt.plane).unwrap();
			status.insert(evt.id, PlayerStatus::Alive).unwrap();
		}
	}
}

impl SystemInfo for InitTraits {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

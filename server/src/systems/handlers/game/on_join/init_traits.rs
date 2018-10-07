use specs::*;

use std::time::Instant;

use types::*;

use systems::handlers::game::timer::LoginHandler;
use SystemInfo;

use component::channel::*;
use component::flag::*;
use component::time::*;
use protocol::PlayerStatus;

pub struct InitTraits {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitTraitsData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub start_time: Read<'a, StartTime>,
	pub this_frame: Read<'a, ThisFrame>,

	pub score: WriteStorage<'a, Score>,
	pub level: WriteStorage<'a, Level>,
	pub team: WriteStorage<'a, Team>,
	pub plane: WriteStorage<'a, Plane>,
	pub status: WriteStorage<'a, Status>,
	pub session: WriteStorage<'a, Session>,
	pub flag: WriteStorage<'a, FlagCode>,
	pub is_player: WriteStorage<'a, IsPlayer>,
	pub pingdata: WriteStorage<'a, PingData>,
	pub lastshot: WriteStorage<'a, LastShotTime>,
	pub lastupdate: WriteStorage<'a, LastUpdate>,
	pub last_key: WriteStorage<'a, LastKeyTime>,
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
			start_time,
			this_frame,

			mut score,
			mut level,
			mut team,
			mut plane,
			mut status,
			mut session,
			mut flag,
			mut lastupdate,
			mut is_player,
			mut pingdata,
			mut lastshot,
			mut last_key,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			score.insert(evt.id, Score(0)).unwrap();
			level.insert(evt.id, evt.level).unwrap();
			team.insert(evt.id, evt.team).unwrap();
			plane.insert(evt.id, evt.plane).unwrap();
			status.insert(evt.id, PlayerStatus::Alive).unwrap();
			session.insert(evt.id, evt.session.clone()).unwrap();
			flag.insert(evt.id, evt.flag).unwrap();

			lastupdate
				.insert(evt.id, LastUpdate(Instant::now()))
				.unwrap();
			is_player.insert(evt.id, IsPlayer).unwrap();
			pingdata.insert(evt.id, PingData::default()).unwrap();
			lastshot.insert(evt.id, LastShotTime(start_time.0)).unwrap();
			last_key.insert(evt.id, LastKeyTime(this_frame.0)).unwrap();
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

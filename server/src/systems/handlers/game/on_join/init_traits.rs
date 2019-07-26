use specs::*;

use std::time::Instant;

use crate::types::*;
use crate::SystemInfo;

use crate::component::event::*;
use crate::component::flag::*;
use crate::component::time::*;
use crate::protocol::PlayerStatus;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitTraits;

#[derive(SystemData)]
pub struct InitTraitsData<'a> {
	start_time: Read<'a, StartTime>,
	this_frame: Read<'a, ThisFrame>,

	score: WriteStorage<'a, Score>,
	level: WriteStorage<'a, Level>,
	team: WriteStorage<'a, Team>,
	plane: WriteStorage<'a, Plane>,
	status: WriteStorage<'a, Status>,
	session: WriteStorage<'a, Session>,
	flag: WriteStorage<'a, FlagCode>,
	is_player: WriteStorage<'a, IsPlayer>,
	pingdata: WriteStorage<'a, PingData>,
	lastshot: WriteStorage<'a, LastShotTime>,
	lastupdate: WriteStorage<'a, LastUpdate>,
	last_key: WriteStorage<'a, LastKeyTime>,
}

impl EventHandlerTypeProvider for InitTraits {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitTraits {
	type SystemData = InitTraitsData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.score.insert(evt.id, Score(0)).unwrap();
		data.level.insert(evt.id, evt.level).unwrap();
		data.team.insert(evt.id, evt.team).unwrap();
		data.plane.insert(evt.id, evt.plane).unwrap();
		data.status.insert(evt.id, PlayerStatus::Alive).unwrap();
		data.session.insert(evt.id, evt.session.clone()).unwrap();
		data.flag.insert(evt.id, evt.flag).unwrap();

		data.lastupdate
			.insert(evt.id, LastUpdate(Instant::now()))
			.unwrap();
		data.is_player.insert(evt.id, IsPlayer).unwrap();
		data.pingdata.insert(evt.id, PingData::default()).unwrap();
		data.lastshot
			.insert(evt.id, LastShotTime(data.start_time.0))
			.unwrap();
		data.last_key
			.insert(evt.id, LastKeyTime(data.this_frame.0))
			.unwrap();
	}
}

impl SystemInfo for InitTraits {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

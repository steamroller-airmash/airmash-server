use airmash_protocol::client::Login;
use airmash_protocol::FlagCode;
use specs::*;
use uuid::Uuid;

use std::convert::TryFrom;
use std::str::FromStr;

use component::channel::*;
use component::collection::PlayerNames;
use component::event::PlayerJoin;
use component::time::*;
use consts::timer::*;
use types::*;

use GameMode;

use rand;
use rand::distributions::{IndependentSample, Range};

#[derive(SystemData)]
pub struct LoginSystemData<'a> {
	pub entities: Entities<'a>,
	pub conns: Read<'a, Connections>,
	pub player_names: Write<'a, PlayerNames>,

	pub startime: Read<'a, StartTime>,
	pub player_join: Write<'a, OnPlayerJoin>,
	pub config: Read<'a, Config>,
	pub gamemode: GameModeWriter<'a, GameMode>,
}

pub struct LoginHandler {
	reader: Option<OnTimerEventReader>,
}

impl LoginHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}

	fn do_login<'a>(data: &mut LoginSystemData<'a>, conn: ConnectionId, login: Login) {
		let entity = data.entities.create();

		if entity.id() > 0xFFFF {
			error!(
				target: "server",
				"Entity created with id greater than 0xFFFF. Aborting to avoid sending invalid packets."
			);
			panic!("Entity created with invalid id.");
		}

		info!(
			target: "server",
			"{:?} logging on as {} with id {}",
			conn, login.name, entity.id()
		);

		let flag = {
			let flag_str: &str = &login.flag;
			FlagCode::try_from(flag_str).unwrap_or(FlagCode::UnitedNations)
		};

		let session = match Uuid::from_str(&login.session) {
			Ok(s) => Some(s),
			Err(_) => None,
		};

		let team = data.gamemode.get_mut().assign_team(entity);
		let plane = data.gamemode.get_mut().assign_plane(entity, team);

		let mut name = login.name;
		let range = Range::new(0, 1000);
		let mut rng = rand::thread_rng();
		while data.player_names.0.contains(&name) {
			name = format!("{}#{:03}", name, range.ind_sample(&mut rng));
		}

		data.player_names.0.insert(name.clone(), entity);

		name.truncate(255);
		// Avoid carrying around extra bytes on what
		// should be an immutable string
		name.shrink_to_fit();

		data.player_join.single_write(PlayerJoin {
			id: entity,
			level: Level(0),
			name: Name(name),
			session: Session(session),
			flag: flag,
			team,
			plane,
			conn,
		});
	}
}

impl<'a> System<'a> for LoginHandler {
	type SystemData = (Read<'a, OnTimerEvent>, LoginSystemData<'a>);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (channel, mut data): Self::SystemData) {
		for evt in channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *LOGIN_PASSED {
				continue;
			}

			let evt = match evt.data {
				Some(ref v) => match (*v).downcast_ref::<(ConnectionId, Login)>() {
					Some(v) => v.clone(),
					None => continue,
				},
				None => continue,
			};

			Self::do_login(&mut data, evt.0, evt.1);
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for LoginHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

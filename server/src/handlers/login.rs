use airmash_protocol::client::Login;
use airmash_protocol::{FlagCode, PlaneType};
use specs::*;
use uuid::Uuid;

use std::str::FromStr;
use std::time::Instant;

use component::channel::*;
use component::counter::PlayersGame;
use component::event::PlayerJoin;
use component::time::*;
use types::*;
use utils::geoip;

use GameMode;

// Login needs write access to just
// about everything
#[derive(SystemData)]
pub struct LoginSystemData<'a> {
	pub entities: Entities<'a>,
	pub energy: WriteStorage<'a, Energy>,
	pub health: WriteStorage<'a, Health>,
	pub keystate: WriteStorage<'a, KeyState>,
	pub name: WriteStorage<'a, Name>,
	pub session: WriteStorage<'a, Session>,
	pub powerups: WriteStorage<'a, Powerups>,
	pub upgrades: WriteStorage<'a, Upgrades>,
	pub level: WriteStorage<'a, Level>,
	pub flag: WriteStorage<'a, Flag>,
	pub conns: Write<'a, Connections>,
	pub associated_conn: WriteStorage<'a, AssociatedConnection>,
	pub lastupdate: WriteStorage<'a, LastUpdate>,
	pub isplayer: WriteStorage<'a, IsPlayer>,
	pub pingdata: WriteStorage<'a, PingData>,
	pub playersgame: Write<'a, PlayersGame>,
	pub lastshot: WriteStorage<'a, LastShotTime>,
	pub energyregen: WriteStorage<'a, EnergyRegen>,

	pub startime: Read<'a, StartTime>,
	pub player_join: Write<'a, OnPlayerJoin>,
	pub config: Read<'a, Config>,
	pub gamemode: GameModeWriter<'a, GameMode>,
}

struct LoginInfo<'a> {
	pub id: Entity,
	pub login: &'a Login,
	pub flag: FlagCode,
	pub team: Team,
	pub plane: Plane,
	pub pos: Position
}

pub struct LoginHandler {
	reader: Option<OnLoginReader>,
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

		let flag = match FlagCode::from_str(&login.flag) {
			Some(v) => v,
			None => geoip::locate(&data.conns.0[&conn].addr).unwrap_or(FlagCode::UnitedNations),
		};

		let session = match Uuid::from_str(&login.session) {
			Ok(s) => Some(s),
			Err(_) => None,
		};

		data.conns.associate(conn, entity, ConnectionType::Primary);

		// Set all possible pieces of state for a plane
		data.energy.insert(entity, Energy::new(1.0)).unwrap();
		data.health.insert(entity, Health::new(1.0)).unwrap();
		data.keystate.insert(entity, KeyState::default()).unwrap();
		data.name.insert(entity, Name(login.name)).unwrap();
		data.session.insert(entity, Session(session)).unwrap();
		data.powerups.insert(entity, Powerups::default()).unwrap();
		data.upgrades.insert(entity, Upgrades::default()).unwrap();
		data.level.insert(entity, Level(0)).unwrap();
		data.flag.insert(entity, flag).unwrap();
		data.associated_conn
			.insert(entity, AssociatedConnection(conn))
			.unwrap();
		data.lastupdate
			.insert(entity, LastUpdate(Instant::now()))
			.unwrap();
		data.isplayer.insert(entity, IsPlayer {}).unwrap();
		data.pingdata.insert(entity, PingData::default()).unwrap();
		data.lastshot
			.insert(entity, LastShotTime(data.startime.0))
			.unwrap();
		data.energyregen
			.insert(entity, data.config.planes[PlaneType::Predator].energy_regen)
			.unwrap();

		data.playersgame.0 += 1;
		data.player_join.single_write(PlayerJoin(entity));
	}
}

impl<'a> System<'a> for LoginHandler {
	type SystemData = (Read<'a, OnLogin>, LoginSystemData<'a>);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(res.fetch_mut::<OnLogin>().register_reader());

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (channel, mut data): Self::SystemData) {
		if let Some(ref mut reader) = self.reader {
			for evt in channel.read(reader).cloned() {
				Self::do_login(&mut data, evt.0, evt.1);
			}
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

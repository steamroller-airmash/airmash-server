
pub mod server {
	use protocol::datatypes::*;
	use protocol::server::*;
	use protocol::ServerPacket;
	use specs::Entity;
	use types::{
		Position,
		Rotation,
		Velocity,
		Energy,
		Health,
		EnergyRegen,
		HealthRegen,
		Accel,
		Level,
		Speed,
		Team
	};
	
	use std::borrow::Cow;

	pub fn login_player<'a>(
		id: Entity,
		status: PlayerStatus,
		level: Level,
		name: Cow<'a, str>,
		ty: PlaneType,
		team: Team,
		pos: Position,
		rot: Rotation,
		flag: FlagCode,
		upgrades: Upgrades
	) -> LoginPlayer {
		assert!(id.id() < 0xFFFF);

		LoginPlayer {
			id: id.id() as u16,
			status,
			level: level.0,
			name: name.into_owned(),
			ty,
			team: team.0,
			pos_x: pos.x.inner(),
			pos_y: pos.y.inner(),
			rot: rot,
			flag,
			upgrades
		}
	}

	pub fn login<'a, 'b>(
		success: bool,
		id: Entity,
		team: Team,
		clock: u32,
		token: Cow<'a, str>,
		ty: PlaneType,
		room: Cow<'a, str>,
		players: Vec<LoginPlayer>
	) -> ServerPacket {
		assert!(id.id() < 0xFFFF);

		ServerPacket::Login(Login {
			success,
			id: id.id() as u16,
			team: team.0,
			clock,
			token: token.into_owned(),
			ty,
			room: room.into_owned(),
			players
		})
	}

	pub fn backup() -> ServerPacket {
		ServerPacket::Backup
	}

	pub fn ping(clock: u32, num: u32) -> ServerPacket {
		ServerPacket::Ping(Ping { clock, num })
	}

	pub fn ping_result(
		ping: u16,
		players_total: u32,
		players_game: u32
	) -> ServerPacket {
		ServerPacket::PingResult(PingResult {
			ping, players_total, players_game
		})
	}

	pub fn command_reply<'a>(
		ty: u8,
		text: Cow<'a, str>
	) -> ServerPacket {
		ServerPacket::CommandReply(CommandReply {
			ty,
			text: text.into_owned()
		})
	}

	pub fn player_new<'a>(
		id: Entity,
		status: PlayerStatus,
		name: Cow<'a, str>,
		team: Team,
		ty: PlaneType,
		pos: Position,
		rot: Rotation,
		flag: FlagCode,
		upgrades: Upgrades
	) -> ServerPacket {
		assert!(id.id() < 0xFFFF);

		ServerPacket::PlayerNew(PlayerNew {
			id: id.id() as u16,
			status,
			name: name.into_owned(),
			ty,
			team: team.0,
			pos_x: pos.x.inner(),
			pos_y: pos.y.inner(),
			rot: rot,
			flag,
			upgrades
		})
	}

	pub fn player_leave(id: Entity) -> ServerPacket {
		assert!(id.id() < 0xFFFF);

		ServerPacket::PlayerLeave(PlayerLeave {
			id: id.id() as u16
		})
	}

	pub fn player_update(
		clock: u32,
		id: Entity,
		keystate: ServerKeyState,
		upgrades: Upgrades,
		pos: Position,
		rot: Rotation,
		vel: Velocity
	) -> ServerPacket {
		assert!(id.id() < 0xFFFF);

		ServerPacket::PlayerUpdate(PlayerUpdate {
			clock,
			id: id.id() as u16,
			keystate,
			upgrades,
			pos_x: pos.x.inner(),
			pos_y: pos.y.inner(),
			rot: rot,
			speed_x: vel.x.inner(),
			speed_y: vel.y.inner()
		})
	}

	pub fn player_fire_projectile(
		id: Entity,
		ty: MobType,
		pos: Position,
		speed: Velocity,
		accel: Accel,
		max_speed: Speed
	) -> PlayerFireProjectile {
		assert!(id.id() < 0xFFFF);

		PlayerFireProjectile {
			id: id.id() as u16,
			ty,
			pos_x: pos.x.inner(),
			pos_y: pos.y.inner(),
			speed_x: speed.x.inner(),
			speed_y: speed.y.inner(),
			accel_x: accel.x.inner(),
			accel_y: accel.y.inner(),
			max_speed: max_speed.inner()
		}
	}

	pub fn player_fire(
		clock: u32,
		id: Entity,
		energy: Energy,
		energy_regen: EnergyRegen,
		projectiles: Vec<PlayerFireProjectile>
	) -> ServerPacket {
		assert!(id.id() < 0xFFFF);

		ServerPacket::PlayerFire(PlayerFire {
			clock,
			id: id.id() as u16,
			energy: energy.inner(),
			energy_regen: energy_regen.inner(),
			projectiles
		})
	}

	pub fn player_respawn(
		id: Entity,
		pos: Position,
		rot: Rotation,
		upgrades: Upgrades
	) -> ServerPacket {
		assert!(id.id() < 0xFFFF);

		ServerPacket::PlayerRespawn(PlayerRespawn {
			id: id.id() as u16,
			pos_x: pos.x.inner(),
			pos_y: pos.y.inner(),
			rot: rot,
			upgrades
		})
	}
}

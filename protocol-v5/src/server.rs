use error::*;
use protocol_common::*;
use serde::*;

impl_serde! {
	struct server::LoginPlayer {
		id: Player,
		status: PlayerStatus,
		level: Level,
		name: text,
		ty: PlaneType,
		team: Team,
		pos: position,
		rot: rotation,
		flag: FlagCode,
		upgrades: Upgrades
	}

	struct server::Login {
		success: bool,
		id: Player,
		team: Team,
		clock: u32,
		token: text,
		ty: GameType,
		room: text,
		players: array_large
	}

	struct server::Ping {
		clock: u32,
		num: u32
	}

	struct server::PingResult {
		ping: u16,
		players_total: u32,
		players_game: u32
	}

	struct server::CommandReply {
		ty: CommandReplyType,
		text: text_big
	}

	struct server::PlayerNew {
		id: Player,
		status: PlayerStatus,
		name: text,
		ty: PlaneType,
		team: Team,
		pos: position,
		rot: rotation,
		flag: FlagCode,
		upgrades: Upgrades
	}

	struct server::PlayerLeave {
		id: Player,
	}

	struct server::PlayerUpdate {
		clock: u32,
		id: Player,
		keystate: ServerKeyState,
		upgrades: Upgrades,
		pos: position24,
		rot: rotation,
		speed: velocity
	}

	struct server::PlayerFireProjectile {
		id: Mob,
		ty: MobType,
		pos: position,
		speed: velocity,
		accel: accel,
		max_speed: speed
	}

	struct server::PlayerFire {
		clock: u32,
		id: Player,
		energy: energy,
		energy_regen: energy_regen,
		projectiles: array_small
	}

	struct server::PlayerRespawn {
		id: Player,
		pos: position24,
		rot: rotation,
		upgrades: Upgrades
	}

	struct server::PlayerFlag {
		id: Player,
		flag: FlagCode
	}

	struct server::PlayerLevel {
		id: Player,
		ty: PlayerLevelType,
		level: Level
	}

	struct server::PlayerHitPlayer {
		id: Player,
		health: health,
		health_regen: health_regen
	}

	struct server::PlayerHit {
		id: Mob,
		ty: MobType,
		pos: position,
		owner: Player,
		players: array_small
	}

	struct server::PlayerKill {
		id: Player,
		killer: option_player,
		pos: position
	}

	struct server::PlayerUpgrade {
		upgrades: Upgrades,
		ty: UpgradeType,
		speed: u8,
		defense: u8,
		energy: u8,
		missile: u8
	}

	struct server::PlayerType {
		id: Player,
		ty: PlaneType
	}

	struct server::PlayerPowerup {
		ty: PowerupType,
		duration: u32,
	}

	struct server::PlayerReteamPlayer {
		id: Player,
		team: Team
	}

	struct server::PlayerReteam {
		players: array_large
	}

	struct server::GameFlag {
		ty: FlagUpdateType,
		flag: Flag,
		id: option_player,
		pos: position24,
		blueteam: u8,
		redteam: u8
	}

	struct server::GameSpectate {
		id: Player
	}

	struct server::GamePlayersAlive {
		players: u16,
	}

	struct server::GameFirewall {
		ty: FirewallUpdateType,
		status: FirewallStatus,
		pos: position,
		radius: f32,
		speed: f32
	}

	struct server::EventRepelPlayer {
		id: Player,
		keystate: ServerKeyState,
		pos: position,
		rot: rotation,
		speed: velocity,
		energy: energy,
		energy_regen: energy_regen,
		health: health,
		health_regen: health_regen
	}

	struct server::EventRepelMob {
		id: Mob,
		ty: MobType,
		pos: position,
		speed: velocity,
		accel: accel,
		max_speed: speed
	}

	struct server::EventRepel {
		clock: u32,
		id: Player,
		pos: position,
		rot: rotation,
		speed: velocity,
		energy: energy,
		energy_regen: energy_regen,
		players: array_small,
		mobs: array_small
	}

	struct server::EventBoost {
		clock: u32,
		id: Player,
		boost: bool,
		pos: position24,
		rot: rotation,
		speed: velocity,
		energy: energy,
		energy_regen: energy_regen
	}

	struct server::EventBounce {
		clock: u32,
		id: Player,
		keystate: ServerKeyState,
		pos: position24,
		rot: rotation,
		speed: velocity
	}

	struct server::EventStealth {
		id: Player,
		state: bool,
		energy: energy,
		energy_regen: energy_regen
	}

	struct server::EventLeaveHorizon {
		ty: LeaveHorizonType,
		id: u16
	}

	struct server::MobUpdate {
		clock: u32,
		id: Mob,
		ty: MobType,
		pos: position,
		speed: velocity,
		accel: accel,
		max_speed: speed
	}

	struct server::MobUpdateStationary {
		id: Mob,
		ty: MobType,
		pos: position_f32,
	}

	struct server::MobDespawn {
		id: Mob,
		ty: MobType
	}

	struct server::MobDespawnCoords {
		id: Mob,
		ty: MobType,
		pos: position,
	}

	struct server::ScoreUpdate {
		id: Player,
		score: Score,
		earnings: Score,
		upgrades: u16,
		total_kills: u32,
		total_deaths: u32
	}

	struct server::ScoreBoardData {
		id: Player,
		score: Score,
		level: Level
	}

	struct server::ScoreBoardRanking {
		id: Player,
		pos: low_res_pos,
	}

	struct server::ScoreBoard {
		data: array_large,
		rankings: array_large
	}

	struct server::ScoreDetailedFFAEntry {
		id: Player,
		level: Level,
		score: Score,
		kills: u16,
		deaths: u16,
		damage: f32,
		ping: u16
	}

	struct server::ScoreDetailedFFA {
		scores: array_large
	}

	struct server::ScoreDetailedCTFEntry {
		id: Player,
		level: Level,
		captures: u16,
		score: Score,
		kills: u16,
		deaths: u16,
		damage: f32,
		ping: u16
	}

	struct server::ScoreDetailedCTF {
		scores: array_large
	}

	struct server::ScoreDetailedBTREntry {
		id: Player,
		level: Level,
		alive: bool,
		wins: u16,
		score: Score,
		kills: u16,
		deaths: u16,
		damage: f32,
		ping: u16
	}

	struct server::ScoreDetailedBTR {
		scores: array_large
	}

	struct server::ChatTeam {
		id: Player,
		text: text
	}

	struct server::ChatPublic {
		id: Player,
		text: text
	}

	struct server::ChatSay {
		id: Player,
		text: text
	}

	struct server::ChatWhisper {
		from: Player,
		to: Player,
		text: text
	}

	struct server::ChatVoteMutePassed {
		id: Player
	}

	struct server::ServerMessage {
		ty: ServerMessageType,
		duration: u32,
		text: text_big
	}

	struct server::ServerCustom {
		ty: ServerCustomType,
		data: text_big
	}

	struct server::Error {
		error: ErrorType
	}
}

mod consts {
	pub const LOGIN: u8 = 0;
	pub const BACKUP: u8 = 1;
	pub const PING: u8 = 5;
	pub const PING_RESULT: u8 = 6;
	pub const ACK: u8 = 7;
	pub const ERROR: u8 = 8;
	pub const COMMAND_REPLY: u8 = 9;
	pub const PLAYER_NEW: u8 = 10;
	pub const PLAYER_LEAVE: u8 = 11;
	pub const PLAYER_UPDATE: u8 = 12;
	pub const PLAYER_FIRE: u8 = 13;
	pub const PLAYER_HIT: u8 = 14;
	pub const PLAYER_RESPAWN: u8 = 15;
	pub const PLAYER_FLAG: u8 = 16;
	pub const PLAYER_KILL: u8 = 17;
	pub const PLAYER_UPGRADE: u8 = 18;
	pub const PLAYER_TYPE: u8 = 19;
	pub const PLAYER_POWERUP: u8 = 20;
	pub const PLAYER_LEVEL: u8 = 21;
	pub const PLAYER_RETEAM: u8 = 22;
	pub const GAME_FLAG: u8 = 30;
	pub const GAME_SPECTATE: u8 = 31;
	pub const GAME_PLAYERSALIVE: u8 = 32;
	pub const GAME_FIREWALL: u8 = 33;
	pub const EVENT_REPEL: u8 = 40;
	pub const EVENT_BOOST: u8 = 41;
	pub const EVENT_BOUNCE: u8 = 42;
	pub const EVENT_STEALTH: u8 = 43;
	pub const EVENT_LEAVEHORIZON: u8 = 44;
	pub const MOB_UPDATE: u8 = 60;
	pub const MOB_UPDATE_STATIONARY: u8 = 61;
	pub const MOB_DESPAWN: u8 = 62;
	pub const MOB_DESPAWN_COORDS: u8 = 63;
	pub const CHAT_PUBLIC: u8 = 70;
	pub const CHAT_TEAM: u8 = 71;
	pub const CHAT_SAY: u8 = 72;
	pub const CHAT_WHISPER: u8 = 73;
	pub const CHAT_VOTEMUTEPASSED: u8 = 78;
	pub const CHAT_VOTEMUTED: u8 = 79;
	pub const SCORE_UPDATE: u8 = 80;
	pub const SCORE_BOARD: u8 = 81;
	pub const SCORE_DETAILED_FFA: u8 = 82;
	pub const SCORE_DETAILED_CTF: u8 = 83;
	pub const SCORE_DETAILED_BTR: u8 = 84;
	pub const SERVER_MESSAGE: u8 = 90;
	pub const SERVER_CUSTOM: u8 = 91;
}

impl Serialize for ServerPacket {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		use self::consts::*;
		use self::ServerPacket::*;

		match self {
			Login(x) => (LOGIN, x).serialize(ser),
			Backup => BACKUP.serialize(ser),
			Ping(x) => (PING, x).serialize(ser),
			PingResult(x) => (PING_RESULT, x).serialize(ser),
			Ack => ACK.serialize(ser),
			Error(x) => (ERROR, x).serialize(ser),
			CommandReply(x) => (COMMAND_REPLY, x).serialize(ser),
			PlayerNew(x) => (PLAYER_NEW, x).serialize(ser),
			PlayerLeave(x) => (PLAYER_LEAVE, x).serialize(ser),
			PlayerUpdate(x) => (PLAYER_UPDATE, x).serialize(ser),
			PlayerFire(x) => (PLAYER_FIRE, x).serialize(ser),
			PlayerRespawn(x) => (PLAYER_RESPAWN, x).serialize(ser),
			PlayerFlag(x) => (PLAYER_FLAG, x).serialize(ser),
			PlayerHit(x) => (PLAYER_HIT, x).serialize(ser),
			PlayerKill(x) => (PLAYER_KILL, x).serialize(ser),
			PlayerUpgrade(x) => (PLAYER_UPGRADE, x).serialize(ser),
			PlayerType(x) => (PLAYER_TYPE, x).serialize(ser),
			PlayerPowerup(x) => (PLAYER_POWERUP, x).serialize(ser),
			PlayerLevel(x) => (PLAYER_LEVEL, x).serialize(ser),
			PlayerReteam(x) => (PLAYER_RETEAM, x).serialize(ser),
			GameFlag(x) => (GAME_FLAG, x).serialize(ser),
			GameSpectate(x) => (GAME_SPECTATE, x).serialize(ser),
			GamePlayersAlive(x) => (GAME_PLAYERSALIVE, x).serialize(ser),
			GameFirewall(x) => (GAME_FIREWALL, x).serialize(ser),
			EventRepel(x) => (EVENT_REPEL, x).serialize(ser),
			EventBoost(x) => (EVENT_BOOST, x).serialize(ser),
			EventBounce(x) => (EVENT_BOUNCE, x).serialize(ser),
			EventStealth(x) => (EVENT_STEALTH, x).serialize(ser),
			EventLeaveHorizon(x) => (EVENT_LEAVEHORIZON, x).serialize(ser),
			MobUpdate(x) => (MOB_UPDATE, x).serialize(ser),
			MobUpdateStationary(x) => (MOB_UPDATE_STATIONARY, x).serialize(ser),
			MobDespawn(x) => (MOB_DESPAWN, x).serialize(ser),
			MobDespawnCoords(x) => (MOB_DESPAWN_COORDS, x).serialize(ser),
			ScoreUpdate(x) => (SCORE_UPDATE, x).serialize(ser),
			ScoreBoard(x) => (SCORE_BOARD, x).serialize(ser),
			ScoreDetailedFFA(x) => (SCORE_DETAILED_FFA, x).serialize(ser),
			ScoreDetailedCTF(x) => (SCORE_DETAILED_CTF, x).serialize(ser),
			ScoreDetailedBTR(x) => (SCORE_DETAILED_BTR, x).serialize(ser),
			ChatTeam(x) => (CHAT_TEAM, x).serialize(ser),
			ChatPublic(x) => (CHAT_PUBLIC, x).serialize(ser),
			ChatSay(x) => (CHAT_SAY, x).serialize(ser),
			ChatWhisper(x) => (CHAT_WHISPER, x).serialize(ser),
			ChatVoteMutePassed(x) => (CHAT_VOTEMUTEPASSED, x).serialize(ser),
			ChatVoteMuted => CHAT_VOTEMUTED.serialize(ser),
			ServerMessage(x) => (SERVER_MESSAGE, x).serialize(ser),
			ServerCustom(x) => (SERVER_CUSTOM, x).serialize(ser),
		}
	}
}

macro_rules! match_case {
	($ty:ident, $de:ident) => {
		$ty(Deserialize::deserialize($de).map_err(|e| {
			e.chain(FieldSpec {
				field: FieldName::Name(stringify!($ty)),
				ty: "ServerPacket".into(),
				})
		})?).into()
	};
}

impl Deserialize for ServerPacket {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		use self::consts::*;
		use self::ServerPacket::*;

		let val = de.deserialize_u8().map_err(|e| {
			e.chain(FieldSpec {
				field: FieldName::Name("<variant-number>"),
				ty: "ServerPacket".into(),
			})
		})?;

		Ok(match val {
			LOGIN => match_case!(Login, de),
			BACKUP => Backup.into(),
			PING => match_case!(Ping, de),
			PING_RESULT => match_case!(PingResult, de),
			ACK => Ack.into(),
			ERROR => match_case!(Error, de),
			COMMAND_REPLY => match_case!(CommandReply, de),
			PLAYER_NEW => match_case!(PlayerNew, de),
			PLAYER_LEAVE => match_case!(PlayerLeave, de),
			PLAYER_UPDATE => match_case!(PlayerUpdate, de),
			PLAYER_FIRE => match_case!(PlayerFire, de),
			PLAYER_RESPAWN => match_case!(PlayerRespawn, de),
			PLAYER_FLAG => match_case!(PlayerFlag, de),
			PLAYER_HIT => match_case!(PlayerHit, de),
			PLAYER_KILL => match_case!(PlayerKill, de),
			PLAYER_UPGRADE => match_case!(PlayerUpgrade, de),
			PLAYER_TYPE => match_case!(PlayerType, de),
			PLAYER_POWERUP => match_case!(PlayerPowerup, de),
			PLAYER_LEVEL => match_case!(PlayerLevel, de),
			PLAYER_RETEAM => match_case!(PlayerReteam, de),
			GAME_FLAG => match_case!(GameFlag, de),
			GAME_SPECTATE => match_case!(GameSpectate, de),
			GAME_PLAYERSALIVE => match_case!(GamePlayersAlive, de),
			GAME_FIREWALL => match_case!(GameFirewall, de),
			EVENT_REPEL => match_case!(EventRepel, de),
			EVENT_BOOST => match_case!(EventBoost, de),
			EVENT_BOUNCE => match_case!(EventBounce, de),
			EVENT_STEALTH => match_case!(EventStealth, de),
			EVENT_LEAVEHORIZON => match_case!(EventLeaveHorizon, de),
			MOB_UPDATE => match_case!(MobUpdate, de),
			MOB_UPDATE_STATIONARY => match_case!(MobUpdateStationary, de),
			MOB_DESPAWN => match_case!(MobDespawn, de),
			MOB_DESPAWN_COORDS => match_case!(MobDespawnCoords, de),
			SCORE_UPDATE => match_case!(ScoreUpdate, de),
			SCORE_BOARD => match_case!(ScoreBoard, de),
			SCORE_DETAILED_BTR => match_case!(ScoreDetailedBTR, de),
			SCORE_DETAILED_FFA => match_case!(ScoreDetailedFFA, de),
			SCORE_DETAILED_CTF => match_case!(ScoreDetailedCTF, de),
			CHAT_TEAM => match_case!(ChatTeam, de),
			CHAT_PUBLIC => match_case!(ChatPublic, de),
			CHAT_SAY => match_case!(ChatSay, de),
			CHAT_WHISPER => match_case!(ChatWhisper, de),
			CHAT_VOTEMUTEPASSED => match_case!(ChatVoteMutePassed, de),
			CHAT_VOTEMUTED => ChatVoteMuted.into(),
			SERVER_MESSAGE => match_case!(ServerMessage, de),
			SERVER_CUSTOM => match_case!(ServerCustom, de),
			x => {
				return Err(DeserializeError {
					ty: DeserializeErrorType::InvalidEnumValue(x as usize),
					trace: vec![FieldSpec {
						field: FieldName::Name("<variant-number>"),
						ty: "ServerPacket".into(),
					}],
				});
			}
		})
	}
}

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

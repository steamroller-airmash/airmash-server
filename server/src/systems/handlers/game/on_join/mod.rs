mod init_earnings;
mod init_join_time;
mod init_kill_count;
mod init_last_repel_time;
mod init_state;
mod init_stealth_time;
mod init_traits;
mod init_transform;
mod send_level;
mod send_login;
mod send_player_new;
mod send_score;

pub use self::init_earnings::InitEarnings;
pub use self::init_join_time::InitJoinTime;
pub use self::init_kill_count::InitKillCounters;
pub use self::init_last_repel_time::InitLastRepelTime;
pub use self::init_state::InitState;
pub use self::init_stealth_time::InitStealthTime;
pub use self::init_traits::InitTraits;
pub use self::init_transform::InitTransform;
pub use self::send_level::SendPlayerLevel;
pub use self::send_login::SendLogin;
pub use self::send_player_new::SendPlayerNew;
pub use self::send_score::SendScoreUpdate;

pub type AllJoinHandlers = (
	InitEarnings,
	InitJoinTime,
	InitKillCounters,
	InitLastRepelTime,
	InitState,
	InitStealthTime,
	InitTraits,
	InitTransform,
	SendPlayerLevel,
	SendLogin,
	SendPlayerNew,
	SendScoreUpdate,
);

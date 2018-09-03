macro_rules! impl_from_newtype_inner {
	($enum:tt, $type:tt) => {
		impl From<$type> for $enum {
			fn from(v: $type) -> Self {
				$enum::$type(v)
			}
		}
	};
}

mod client {
	use protocol::client::*;

	macro_rules! impl_from_newtype_client {
		($type:tt) => {
			impl_from_newtype_inner!(ClientPacket, $type);
		};
	}

	impl_from_newtype_client!(Login);
	impl_from_newtype_client!(Backup);
	impl_from_newtype_client!(Horizon);
	impl_from_newtype_client!(Pong);
	impl_from_newtype_client!(Key);
	impl_from_newtype_client!(Command);
	impl_from_newtype_client!(Chat);
	impl_from_newtype_client!(TeamChat);
	impl_from_newtype_client!(Whisper);
	impl_from_newtype_client!(Say);
	impl_from_newtype_client!(VoteMute);
	impl_from_newtype_client!(LocalPing);
}

mod server {
	use protocol::server::*;

	macro_rules! impl_from_newtype_server {
		($type:tt) => {
			impl_from_newtype_inner!(ServerPacket, $type);
		};
	}

	impl_from_newtype_server!(Login);
	impl_from_newtype_server!(Ping);
	impl_from_newtype_server!(PingResult);
	impl_from_newtype_server!(Error);
	impl_from_newtype_server!(CommandReply);
	impl_from_newtype_server!(PlayerNew);
	impl_from_newtype_server!(PlayerLeave);
	impl_from_newtype_server!(PlayerUpdate);
	impl_from_newtype_server!(PlayerFire);
	impl_from_newtype_server!(PlayerRespawn);
	impl_from_newtype_server!(PlayerFlag);
	impl_from_newtype_server!(PlayerHit);
	impl_from_newtype_server!(PlayerKill);
	impl_from_newtype_server!(PlayerUpgrade);
	impl_from_newtype_server!(PlayerType);
	impl_from_newtype_server!(PlayerPowerup);
	impl_from_newtype_server!(PlayerLevel);
	impl_from_newtype_server!(PlayerReteam);
	impl_from_newtype_server!(GameFlag);
	impl_from_newtype_server!(GameSpectate);
	impl_from_newtype_server!(GamePlayersAlive);
	impl_from_newtype_server!(EventRepel);
	impl_from_newtype_server!(EventBoost);
	impl_from_newtype_server!(EventBounce);
	impl_from_newtype_server!(EventStealth);
	impl_from_newtype_server!(EventLeaveHorizon);
	impl_from_newtype_server!(MobUpdate);
	impl_from_newtype_server!(MobUpdateStationary);
	impl_from_newtype_server!(MobDespawn);
	impl_from_newtype_server!(MobDespawnCoords);
	impl_from_newtype_server!(ScoreUpdate);
	impl_from_newtype_server!(ScoreBoard);
	impl_from_newtype_server!(ScoreDetailedFFA);
	impl_from_newtype_server!(ScoreDetailedCTF);
	impl_from_newtype_server!(ScoreDetailedBTR);
	impl_from_newtype_server!(ChatTeam);
	impl_from_newtype_server!(ChatPublic);
	impl_from_newtype_server!(ChatSay);
	impl_from_newtype_server!(ChatWhisper);
	impl_from_newtype_server!(ChatVoteMutePassed);
	impl_from_newtype_server!(ServerMessage);
	impl_from_newtype_server!(ServerCustom);
}

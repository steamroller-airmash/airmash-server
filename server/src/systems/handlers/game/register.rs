use super::*;
use dispatch::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		// Spectate events
		.with::<on_spectate_event::SetSpectateFlag>()
		.with::<on_spectate_event::SendKillPacket>()
		.with::<on_spectate_event::SendSpectatePacket>()
		.with::<on_spectate_event::SendTimerEvent>()
		.with::<on_spectate_event::SetSpectateTarget>()
		.with_handler::<on_spectate_event::CreateDespawnEvent>()
		.with_handler::<on_spectate_event::SetDeadFlag>()
		// On player killed
		.with_handler::<on_player_killed::SetRespawnTimer>()
		.with_handler::<on_player_killed::DisplayMessage>()
		.with_handler::<on_player_killed::UpdateScore>()
		.with_handler::<on_player_killed::CreateDespawnEvent>()
		.with_handler::<on_player_killed::DespawnMissile>()
		// On player joined
		.with::<on_join::InitConnection>()
		.with::<on_join::InitKillCounters>()
		.with::<on_join::InitJoinTime>()
		.with::<on_join::InitEarnings>()
		.with::<on_join::InitTraits>()
		.with::<on_join::InitState>()
		.with::<on_join::InitName>()
		.with::<on_join::InitLimiters>()
		.with::<on_join::InitTransform>()
		.with::<on_join::InitStealthTime>()
		.with::<on_join::InitLastRepelTime>()
		.with::<on_join::SendPlayerNew>()
		.with::<on_join::SendLogin>()
		.with::<on_join::SendPlayerLevel>()
		.with::<on_join::SendScoreUpdate>()
		.with::<on_join::UpdatePlayersGame>()
		.with_handler::<on_join::SendPlayerPowerup>()
		// On player leave
		.with::<on_leave::FreeName>()
		.with::<on_leave::UpdatePlayersGame>()
		.with_handler::<on_leave::CreateDespawnEvent>()
		// On missile fire
		.with_handler::<on_missile_fire::SendPlayerFire>()
		.with_handler::<on_missile_fire::SetLastShot>()
		// On player hit
		.with_handler::<on_player_hit::InflictDamage>()
		.with::<on_player_hit::SendPacket>()
		// On player respawn
		.with_handler::<on_player_respawn::ResetKeyState>()
		.with_handler::<on_player_respawn::SetTraits>()
		.with_handler::<on_player_respawn::SendPlayerRespawn>()
		.with_handler::<on_player_respawn::CreateDespawnEvent>()
		.with_handler::<on_player_respawn::GiveShield>()
		// Chat throttling
		.with_registrar(on_chat_throttled::register)
		// Timer events
		.with_registrar(timer::register)
		// Powerup expiry
		.with_handler::<on_powerup_expire::ForceUpdate>()
		// Powerup Events
		.with_registrar(on_player_powerup::register)
}

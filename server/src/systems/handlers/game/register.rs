use super::*;
use dispatch::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	let builder = builder
		.with::<PlayerKilledCleanup>()

		.with::<on_spectate_event::SetSpectateFlag>()
		.with::<on_spectate_event::SendKillPacket>()
		.with::<on_spectate_event::SendSpectatePacket>()
		.with::<on_spectate_event::SendTimerEvent>()
		.with::<on_spectate_event::SetSpectateTarget>()

		.with::<on_player_killed::SetRespawnTimer>()
		.with::<on_player_killed::DisplayMessage>()
		.with::<on_player_killed::UpdateScore>()

		.with::<on_join::InitKillCounters>()
		.with::<on_join::InitJoinTime>()
		.with::<on_join::InitEarnings>()
		;

	timer::register(builder)
}

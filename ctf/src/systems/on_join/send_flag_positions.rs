use specs::*;

use crate::server::component::event::*;
use crate::server::protocol::server::GameFlag;
use crate::server::protocol::FlagUpdateType;
use crate::server::systems::handlers::game::on_join::SendLogin;
use crate::server::types::systemdata::SendToPlayer;
use crate::server::utils::*;
use crate::server::*;

use crate::component::*;

#[derive(Default)]
pub struct SendFlagPosition;

#[derive(SystemData)]
pub struct SendFlagPositionData<'a> {
	conns: SendToPlayer<'a>,
	scores: Read<'a, GameScores>,

	// These ones are for both
	pos: ReadStorage<'a, Position>,
	team: ReadStorage<'a, Team>,

	// Flag Data
	is_flag: ReadStorage<'a, IsFlag>,
	carrier: ReadStorage<'a, FlagCarrier>,
}

impl EventHandlerTypeProvider for SendFlagPosition {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for SendFlagPosition {
	type SystemData = SendFlagPositionData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		(&data.pos, &data.team, &data.carrier, &data.is_flag)
			.join()
			.for_each(|(pos, team, carrier, _)| {
				let ty = match carrier.0 {
					Some(_) => FlagUpdateType::Carrier,
					None => FlagUpdateType::Position,
				};

				let packet = GameFlag {
					ty,
					flag: Flag(*team),
					pos: *pos,
					id: carrier.0.map(Into::into),
					blueteam: data.scores.blueteam,
					redteam: data.scores.redteam,
				};

				data.conns.send_to_player(evt.id, packet);
			});
	}
}

impl SystemInfo for SendFlagPosition {
	// The client ignores packets that are
	// sent before the login packet
	type Dependencies = SendLogin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

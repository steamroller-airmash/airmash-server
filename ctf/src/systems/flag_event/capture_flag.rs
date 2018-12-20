use server::*;
use specs::*;

use config as ctfconfig;

use component::*;

use server::protocol::server::GameFlag;
use server::protocol::FlagUpdateType;
use server::types::systemdata::*;

#[derive(Default)]
pub struct CaptureFlag;

#[derive(SystemData)]
pub struct CaptureFlagData<'a> {
	ents: Entities<'a>,
	pos: WriteStorage<'a, Position>,
	team: ReadStorage<'a, Team>,
	flag: ReadStorage<'a, IsFlag>,
	carrier: WriteStorage<'a, FlagCarrier>,

	scores: Read<'a, GameScores>,
	channel: Write<'a, OnFlag>,
	conns: SendToAll<'a>,
}

impl<'a> System<'a> for CaptureFlag {
	type SystemData = CaptureFlagData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let mut channel = data.channel;
		let conns = data.conns;
		let scores = *data.scores;

		(
			&mut data.pos,
			&data.team,
			&mut data.carrier,
			&data.flag,
			&*data.ents,
		)
			.join()
			.filter(|(pos, team, carrier, _, _)| {
				// Filter out all flags that aren't within cap radius
				(ctfconfig::FLAG_RETURN_POS[&team] - **pos).length2()
					< *ctfconfig::CAP_RADIUS * *ctfconfig::CAP_RADIUS
					&& carrier.0.is_some()
			})
			.for_each(|(pos, team, carrier, _, ent)| {
				let captor = carrier.0.unwrap();

				*pos = ctfconfig::FLAG_HOME_POS[team];
				*carrier = FlagCarrier(None);

				let blueinc;
				let redinc;

				if *team == ctfconfig::BLUE_TEAM {
					blueinc = 1;
					redinc = 0;
				} else {
					blueinc = 0;
					redinc = 1;
				}

				let packet = GameFlag {
					ty: FlagUpdateType::Position,
					flag: Flag(*team),
					id: None,
					pos: *pos,
					// If both flags are captured at the same time
					// then these scores will be wrong. That's
					// enough of an edge case that we won't deal
					// with it. (Note that this means that the flags
					// were captured within ~16 ms assuming the server
					// is not lagging)
					blueteam: scores.blueteam + blueinc,
					redteam: scores.redteam + redinc,
				};

				conns.send_to_all(packet);

				channel.single_write(FlagEvent {
					ty: FlagEventType::Capture,
					player: Some(captor),
					flag: ent,
				});
			});
	}
}

use systems::PickupFlagSystem;

impl SystemInfo for CaptureFlag {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

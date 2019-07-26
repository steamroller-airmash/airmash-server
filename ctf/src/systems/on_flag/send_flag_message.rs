use crate::server::*;
use specs::*;

use crate::component::*;
use crate::server::protocol::server::GameFlag;
use crate::server::protocol::FlagUpdateType;
use crate::server::types::systemdata::*;
use crate::server::utils::*;

use crate::BLUE_TEAM;
use crate::RED_TEAM;

#[derive(Default)]
pub struct SendFlagMessageSystem;

#[derive(SystemData)]
pub struct SendFlagMessageSystemData<'a> {
	conns: SendToAll<'a>,
	scores: Write<'a, GameScores>,
	flags: ReadExpect<'a, Flags>,

	team: ReadStorage<'a, Team>,
	pos: ReadStorage<'a, Position>,
	carrier: ReadStorage<'a, FlagCarrier>,
}

impl EventHandlerTypeProvider for SendFlagMessageSystem {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for SendFlagMessageSystem {
	type SystemData = SendFlagMessageSystemData<'a>;

	fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
		let ty = match evt.ty {
			FlagEventType::PickUp => FlagUpdateType::Carrier,
			_ => FlagUpdateType::Position,
		};

		let team = try_get!(evt.flag, data.team);

		if evt.ty == FlagEventType::Capture {
			let other;
			if *team == RED_TEAM {
				data.scores.blueteam += 1;
				other = data.flags.blue;
			} else if *team == BLUE_TEAM {
				data.scores.redteam += 1;
				other = data.flags.red;
			} else {
				// Other flags are not implemented for CTF
				// if you are using this code as a base,
				// support for other flags will need to be
				// implemented.
				unimplemented!();
			}

			let flag = Flag(*try_get!(other, data.team));

			if try_get!(other, data.carrier).0.is_none() {
				let pos = *try_get!(other, data.pos);
				data.conns.send_to_all(GameFlag {
					ty,
					flag,
					pos: pos,
					id: None,
					blueteam: data.scores.blueteam,
					redteam: data.scores.redteam,
				});
			} else {
				let carrier = try_get!(other, data.carrier).0.map(|x| x.into());

				data.conns.send_to_all(GameFlag {
					ty: FlagUpdateType::Carrier,
					flag,
					pos: Position::default(),
					id: carrier,
					blueteam: data.scores.blueteam,
					redteam: data.scores.redteam,
				})
			}
		}

		data.conns.send_to_all(GameFlag {
			ty,
			flag: Flag(*team),
			pos: *try_get!(evt.flag, data.pos),
			id: evt.player.map(Into::into),
			blueteam: data.scores.blueteam,
			redteam: data.scores.redteam,
		});
	}
}

use crate::systems::PickupFlagSystem;

impl SystemInfo for SendFlagMessageSystem {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

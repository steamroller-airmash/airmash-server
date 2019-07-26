use crate::server::*;
use specs::*;

use crate::server::component::event::*;
use crate::server::component::time::ThisFrame;
use crate::server::protocol::server::GameFlag;
use crate::server::protocol::FlagUpdateType;
use crate::server::types::systemdata::SendToAll;
use crate::server::utils::*;

use crate::component::*;

#[derive(Default)]
pub struct DropSystem;

#[derive(SystemData)]
pub struct DropSystemData<'a> {
	conns: SendToAll<'a>,
	thisframe: Read<'a, ThisFrame>,

	entities: Entities<'a>,
	pos: WriteStorage<'a, Position>,
	team: ReadStorage<'a, Team>,
	is_flag: ReadStorage<'a, IsFlag>,
	carrier: WriteStorage<'a, FlagCarrier>,
	lastdrop: WriteStorage<'a, LastDrop>,
	flagchannel: Write<'a, OnFlag>,
}

impl EventHandlerTypeProvider for DropSystem {
	type Event = CommandEvent;
}

impl<'a> EventHandler<'a> for DropSystem {
	type SystemData = DropSystemData<'a>;

	fn on_event(&mut self, evt: &CommandEvent, data: &mut Self::SystemData) {
		if evt.1.com != "drop" {
			return;
		}

		let player = match data.conns.associated_player(evt.0) {
			Some(p) => p,
			None => return,
		};

		let thisframe = data.thisframe.0;
		let p_pos = *try_get!(player, data.pos);
		let ref mut flagchannel = data.flagchannel;
		let ref conns = data.conns;

		(
			&mut data.pos,
			&data.team,
			&data.is_flag,
			&mut data.carrier,
			&mut data.lastdrop,
			&*data.entities,
		)
			.join()
			.filter(|(_, _, _, carrier, _, _)| carrier.0.is_some() && carrier.0.unwrap() == player)
			.for_each(|(fpos, team, _, carrier, lastdrop, ent)| {
				let packet = GameFlag {
					ty: FlagUpdateType::Position,
					flag: Flag(*team),
					id: None,
					pos: p_pos,
					blueteam: 0,
					redteam: 0,
				};

				flagchannel.single_write(FlagEvent {
					ty: FlagEventType::Drop,
					player: carrier.0,
					flag: ent,
				});

				*fpos = p_pos;
				*carrier = FlagCarrier(None);
				*lastdrop = LastDrop {
					player: Some(player),
					time: thisframe,
				};

				conns.send_to_all(packet);
			});
	}
}

use super::PickupFlagSystem;

impl SystemInfo for DropSystem {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

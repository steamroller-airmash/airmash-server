
use specs::*;
use types::*;

use systems::ctf::config as ctfconfig;

use component::ctf::*;

use websocket::OwnedMessage;
use protocol::server::{ServerPacket, GameFlag};
use protocol::{to_bytes, FlagUpdateType};

pub struct ReturnFlagSystem;

#[derive(SystemData)]
pub struct ReturnFlagSystemData<'a> {
	pub ents: Entities<'a>,
	pub pos:  WriteStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,
	pub flag: ReadStorage<'a, IsFlag>,
	pub carrier: WriteStorage<'a, FlagCarrier>,

	pub channel: Write<'a, OnCapture>,
	pub conns: Read<'a, Connections>,
}

impl<'a> System<'a> for ReturnFlagSystem {
	type SystemData = ReturnFlagSystemData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let mut channel = data.channel;
		let conns = data.conns;

		(
			&mut data.pos,
			&data.team,
			&mut data.carrier,
			&data.flag,
			&*data.ents
		).join()
			.filter(|(pos, team, carrier, _, _)| {
				// Filter out all flags that aren't within cap radius
				(ctfconfig::FLAG_RETURN_POS[&team] - **pos).length2() < *ctfconfig::CAP_RADIUS * *ctfconfig::CAP_RADIUS
					&& carrier.0.is_some()
			})
			.for_each(|(pos, team, carrier, _, ent)| {
				let captor = carrier.0.unwrap();

				*pos = ctfconfig::FLAG_POS[team];
				*carrier = FlagCarrier(None);

				let packet = GameFlag {
					ty: FlagUpdateType::Position,
					flag: *team,
					id: None,
					pos: *pos,
					blueteam: 0,
					redteam: 0
				};

				conns.send_to_all(OwnedMessage::Binary(
					to_bytes(&ServerPacket::GameFlag(packet)).unwrap()
				));

				channel.single_write(CaptureEvent {
					player: captor,
					flag: ent
				});
			});
	}
}

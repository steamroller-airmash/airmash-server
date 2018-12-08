use specs::prelude::*;
use std::time::Instant;

use types::collision::HitCircle;
use types::connection::{Message, MessageBody, MessageInfo};
use types::*;

use component::collision::PlayerGrid;

use ws::CloseCode;

use protocol::Protocol;
use protocol_v5::ProtocolV5;

use std::mem;
use std::sync::mpsc::{channel, Receiver};

pub struct PollComplete {
	channel: Receiver<Message>,
}

#[derive(SystemData)]
pub struct PollCompleteData<'a> {
	conns: Read<'a, Connections>,
	config: Read<'a, Config>,
	grid: Read<'a, PlayerGrid>,
	entities: Entities<'a>,

	associated: ReadStorage<'a, AssociatedConnection>,
	teams: ReadStorage<'a, Team>,
}

impl PollComplete {
	pub fn new(channel: Receiver<Message>) -> Self {
		Self { channel }
	}
}

impl PollComplete {
	fn send_to_connection<'a>(
		conns: &Read<'a, Connections>,
		id: ConnectionId,
		msg: Option<Vec<u8>>,
	) {
		trace!(target: "airmash:packet-dump", "{:?}", msg);

		match conns.conns.get(&id).map(|ref x| x.sink.clone()) {
			Some(mut conn) => match msg {
				Some(msg) => Connections::send_sink(&mut conn, msg.into()),
				None => conn.close(CloseCode::Normal).unwrap(),
			},
			// The connection probably closed,
			// do nothing
			None => trace!(
					target: "server",
					"Tried to send message to closed connection {:?}",
					id
			),
		}
	}
}

impl<'a> System<'a> for PollComplete {
	type SystemData = PollCompleteData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let conns = data.conns;
		let config = data.config;
		let associated = data.associated;
		let grid = data.grid;
		let teams = data.teams;
		let entities = &*data.entities;
		let protocol = ProtocolV5 {};

		let start = Instant::now();
		while let Ok(msg) = self.channel.try_recv() {
			let data: Option<Vec<u8>> = match msg.msg {
				MessageBody::Packet(ref packet) => {
					Some(protocol.serialize_server(packet).unwrap().next().unwrap())
				}
				MessageBody::Binary(bin) => Some(bin),
				MessageBody::Close => None,
			};

			match msg.info {
				MessageInfo::ToConnection(id) => Self::send_to_connection(&conns, id, data),
				MessageInfo::ToTeam(player) => {
					let player_team = *teams.get(player).unwrap();

					(&associated, &teams)
						.par_join()
						.filter(|(_, team)| **team == player_team)
						.for_each(|(associated, _)| {
							Self::send_to_connection(&conns, associated.0, data.clone());
						});
				}
				MessageInfo::ToVisible(pos) => {
					let ent = entities.entity(0);
					grid.0
						.rough_collide(HitCircle {
							pos: pos,
							rad: config.view_radius,
							layer: 0,
							ent: ent,
						})
						.into_iter()
						.filter_map(|x| associated.get(x))
						.for_each(|associated| {
							Self::send_to_connection(&conns, associated.0, data.clone())
						});
				}
			}
		}

		let time = Instant::now() - start;
		trace!(
			"System {} took {}.{:3} ms",
			Self::name(),
			time.as_secs() * 1000 + time.subsec_millis() as u64,
			time.subsec_nanos() % 1000
		);
	}
}

use dispatch::SystemInfo;
use std::any::Any;

impl SystemInfo for PollComplete {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		unimplemented!();
	}

	fn new_args(mut a: Box<Any>) -> Self {
		let r = a.downcast_mut::<Receiver<Message>>().unwrap();
		// Replace the channel within the box with a
		// dummy one, which will be dropped immediately
		// anyway
		Self::new(mem::replace(r, channel().1))
	}
}

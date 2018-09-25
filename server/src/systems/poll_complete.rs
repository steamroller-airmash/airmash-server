use specs::prelude::*;
use std::time::Instant;
use tokio::prelude::Sink;
use types::connection::{Message, MessageBody, MessageInfo};
use types::*;

use websocket::OwnedMessage;

use protocol::Protocol;
use protocol_v5::ProtocolV5;

use std::mem;
use std::sync::mpsc::{channel, Receiver};

pub struct PollComplete {
	channel: Receiver<Message>,
}

#[derive(SystemData)]
pub struct PollCompleteData<'a> {
	conns: Write<'a, Connections>,

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
		conns: &mut Write<'a, Connections>,
		id: ConnectionId,
		msg: OwnedMessage,
	) {
		trace!(target: "airmash:packet-dump", "{:?}", msg);

		match conns.0.get_mut(&id) {
			Some(ref mut conn) => {
				Connections::send_sink(&mut conn.sink, msg);
			}
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
		let mut conns = data.conns;
		let associated = data.associated;
		let teams = data.teams;
		let protocol = ProtocolV5 {};

		let start = Instant::now();
		let mut cnt = 0;
		while let Ok(msg) = self.channel.try_recv() {
			cnt += 1;

			trace!(
				target: "airmash:packet-dump",
				"Sending packet {:#?} to {:?}",
				msg.msg,
				msg.info
			);

			let data = match msg.msg {
				MessageBody::Packet(ref packet) => {
					OwnedMessage::Binary(protocol.serialize_server(packet).unwrap().next().unwrap())
				}
				MessageBody::Binary(bin) => OwnedMessage::Binary(bin),
				MessageBody::Close => OwnedMessage::Close(None),
			};

			match msg.info {
				MessageInfo::ToConnection(id) => Self::send_to_connection(&mut conns, id, data),
				MessageInfo::ToTeam(player) => {
					let player_team = *teams.get(player).unwrap();

					(&associated, &teams)
						.join()
						.filter(|(_, team)| **team == player_team)
						.for_each(|(associated, _)| {
							Self::send_to_connection(&mut conns, associated.0, data.clone());
						});
				}
				MessageInfo::ToVisible(_player) => {
					// TODO: Implement this properly
					(&associated).join().for_each(|associated| {
						Self::send_to_connection(&mut conns, associated.0, data.clone());
					});
				}
			}
		}

		trace!(
			target: "airmash:packets-sent",
			"Sent {} packets this frame",
			cnt
		);

		for conn in conns.iter_mut() {
			conn.sink
				.poll_complete()
				.map_err(|e| {
					info!("poll_complete failed with error {:?}", e);
				}).err();
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

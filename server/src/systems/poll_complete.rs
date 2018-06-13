use specs::prelude::*;
use tokio::prelude::Sink;
use types::*;

use websocket::OwnedMessage;

use std::sync::mpsc::Receiver;

pub struct PollComplete {
	channel: Receiver<(ConnectionId, OwnedMessage)>,
}

impl PollComplete {
	pub fn new(channel: Receiver<(ConnectionId, OwnedMessage)>) -> Self {
		Self { channel }
	}
}

impl<'a> System<'a> for PollComplete {
	type SystemData = Write<'a, Connections>;

	fn run(&mut self, mut conns: Self::SystemData) {
		while let Ok((id, msg)) = self.channel.try_recv() {
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

		for conn in conns.iter_mut() {
			conn.sink
				.poll_complete()
				.map_err(|e| {
					info!("poll_complete failed with error {:?}", e);
				})
				.err();
		}
	}
}

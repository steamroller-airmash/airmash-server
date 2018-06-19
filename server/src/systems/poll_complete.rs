use specs::prelude::*;
use tokio::prelude::Sink;
use types::*;
use metrics::*;
use std::time::Instant;

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
	type SystemData = (
		Write<'a, Connections>,
		ReadExpect<'a, MetricsHandler>
	);

	fn run(&mut self, (mut conns, metrics): Self::SystemData) {
		let start = Instant::now();
		let mut cnt = 0;
		while let Ok((id, msg)) = self.channel.try_recv() {
			cnt += 1;
			
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

		metrics.count("packets-sent", cnt).unwrap();

		for conn in conns.iter_mut() {
			conn.sink
				.poll_complete()
				.map_err(|e| {
					info!("poll_complete failed with error {:?}", e);
				})
				.err();
		}

		metrics.time_duration("poll-complete", Instant::now() - start).err();
	}
}

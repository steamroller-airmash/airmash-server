
use specs::*;
use types::*;
use consts::SHUTDOWN;

use std::time::{Duration, Instant};
use std::sync::atomic::Ordering;

use websocket::OwnedMessage;
use protocol::{to_bytes, ServerPacket};
use protocol::server::ServerMessage;

#[derive(Default)]
pub struct SignalHandler {
	time: Option<Instant>
}

impl<'a> System<'a> for SignalHandler {
	type SystemData = Read<'a, Connections>;

	fn run(&mut self, data: Self::SystemData) {
		if SHUTDOWN.load(Ordering::Relaxed) {

			if self.time.is_none() {
				self.time = Some(Instant::now());

				let msg = ServerMessage {
					duration: 15,
					ty: 15,
					text: "Server shutting down in 30 seconds!".to_string()
				};

				data.send_to_all(OwnedMessage::Binary(
					to_bytes(&ServerPacket::ServerMessage(msg)).unwrap()
				));

				info!(
					target:"server",
					"Received interrupt, shutting down in 30s"
				);
			}
			else {
				let t = self.time.unwrap();

				if Instant::now() - t > Duration::from_secs(30) {
					panic!("Server shutting down!");
				}
			}
		}
	}
}


use component::counter::PlayersGame;
use component::time::ThisFrame;
use shrev::*;
use specs::*;
use types::*;

use std::time::Instant;

use airmash_protocol::client::Pong;
use airmash_protocol::server::{PingResult, ServerPacket};
use airmash_protocol::to_bytes;
use websocket::OwnedMessage;

pub struct PongHandler {
	reader: Option<ReaderId<(ConnectionId, Pong)>>,
}

#[derive(SystemData)]
pub struct PongHandlerData<'a> {
	channel: Read<'a, EventChannel<(ConnectionId, Pong)>>,
	conns: Read<'a, Connections>,
	thisframe: Read<'a, ThisFrame>,
	playersgame: Read<'a, PlayersGame>,
}

impl PongHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for PongHandler {
	type SystemData = (PongHandlerData<'a>, WriteStorage<'a, PingData>);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<(ConnectionId, Pong)>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (data, mut pingdata): Self::SystemData) {
		let now = Instant::now();

		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = match data.conns.0.get(&evt.0) {
				Some(p) => match p.player {
					Some(p) => p,
					None => continue,
				},
				None => continue,
			};

			let ping = match pingdata
				.get_mut(player)
				.unwrap()
				.receive_ping(evt.1.num, now)
			{
				Some(ping) => ping,
				None => continue,
			};

			let result = PingResult {
				ping: ping.as_millis() as u16,
				players_game: data.playersgame.0,
				players_total: data.playersgame.0,
			};

			data.conns.send_to(
				evt.0,
				OwnedMessage::Binary(to_bytes(&ServerPacket::PingResult(result)).unwrap()),
			);
		}
	}
}

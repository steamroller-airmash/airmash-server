use specs::*;
use types::*;

use std::time::Instant;

use component::channel::{OnPong, OnPongReader};
use component::counter::{PlayerPing, PlayersGame};

use protocol::server::PingResult;

pub struct PongHandler {
	reader: Option<OnPongReader>,
}

#[derive(SystemData)]
pub struct PongHandlerData<'a> {
	channel: Read<'a, OnPong>,
	conns: Read<'a, Connections>,
	playersgame: Read<'a, PlayersGame>,
	ping: WriteStorage<'a, PlayerPing>,
}

impl PongHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for PongHandler {
	type SystemData = (PongHandlerData<'a>, WriteStorage<'a, PingData>);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(res.fetch_mut::<OnPong>().register_reader());

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (mut data, mut pingdata): Self::SystemData) {
		let now = Instant::now();

		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = match data.conns.associated_player(evt.0) {
				Some(p) => p,
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

			data.ping
				.insert(player, PlayerPing(ping.as_millis() as u32))
				.unwrap();

			data.conns.send_to(evt.0, result);
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for PongHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

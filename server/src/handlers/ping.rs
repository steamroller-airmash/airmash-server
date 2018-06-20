use shrev::*;
use specs::*;
use types::*;

use std::time::Instant;

use airmash_protocol::server::Ping as ServerPing;
use airmash_protocol::{to_bytes, ServerPacket};
use websocket::OwnedMessage;

use component::event::PingTimerEvent;
use component::time::*;

pub struct PingTimerHandler {
	reader: Option<ReaderId<PingTimerEvent>>,
}

#[derive(SystemData)]
pub struct PingTimerHandlerData<'a> {
	pub entities: Entities<'a>,
	pub conns: Read<'a, Connections>,
	pub channel: Read<'a, EventChannel<PingTimerEvent>>,
	pub thisframe: Read<'a, ThisFrame>,
	pub starttime: Read<'a, StartTime>,
}

impl PingTimerHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for PingTimerHandler {
	type SystemData = (PingTimerHandlerData<'a>, WriteStorage<'a, PingData>);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<PingTimerEvent>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (data, mut pingdata): Self::SystemData) {
		let clock = (Instant::now() - data.starttime.0).to_clock();

		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			(&*data.entities, &mut pingdata)
				.join()
				.for_each(|(ent, pingdata)| {
					let ping = pingdata.new_ping(data.thisframe.0);

					data.conns.send_to_player(
						ent,
						OwnedMessage::Binary(
							to_bytes(&ServerPacket::Ping(ServerPing {
								clock,
								num: ping.idx,
							})).unwrap(),
						),
					)
				});
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;
use std::any::Any;
use systems::TimerHandler;
impl SystemInfo for PingTimerHandler {
	type Dependencies = (OnCloseHandler, TimerHandler);

	fn new(_: Box<Any>) -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

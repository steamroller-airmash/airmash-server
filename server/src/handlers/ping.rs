use specs::*;
use types::systemdata::*;
use types::*;

use consts::timer::PING_DISPATCH;

use protocol::server::Ping as ServerPing;

use component::channel::{OnTimerEvent, OnTimerEventReader};

pub struct PingTimerHandler {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct PingTimerHandlerData<'a> {
	pub entities: Entities<'a>,
	pub conns: Read<'a, Connections>,
	pub channel: Read<'a, OnTimerEvent>,
	pub clock: ReadClock<'a>,
}

impl PingTimerHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for PingTimerHandler {
	type SystemData = (PingTimerHandlerData<'a>, WriteStorage<'a, PingData>);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (data, mut pingdata): Self::SystemData) {
		let clock = data.clock.get();

		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty == *PING_DISPATCH {
				continue;
			}

			(&*data.entities, &mut pingdata)
				.join()
				.for_each(|(ent, pingdata)| {
					let ping = pingdata.new_ping(data.clock.frame.0);

					data.conns.send_to_player(
						ent,
						ServerPing {
							clock,
							num: ping.idx,
						},
					);
				});
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;
use systems::TimerHandler;

impl SystemInfo for PingTimerHandler {
	type Dependencies = (OnCloseHandler, TimerHandler);

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

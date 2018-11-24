use specs::*;
use types::systemdata::*;
use types::*;

use std::time::{Duration, Instant};

use protocol::server::Ping as ServerPing;
use systems::handlers::game::on_join::SendLogin;

pub struct PingTimerHandler {
	lastping: Instant,
}

#[derive(SystemData)]
pub struct PingTimerHandlerData<'a> {
	entities: Entities<'a>,
	conns: Read<'a, Connections>,
	clock: ReadClock<'a>,
}

impl PingTimerHandler {
	pub fn new() -> Self {
		Self {
			lastping: Instant::now(),
		}
	}
}

impl<'a> System<'a> for PingTimerHandler {
	type SystemData = (PingTimerHandlerData<'a>, WriteStorage<'a, PingData>);

	fn run(&mut self, (data, mut pingdata): Self::SystemData) {
		if data.clock.frame.0 < self.lastping + Duration::from_secs(5) {
			return;
		}
		self.lastping = data.clock.frame.0;

		let clock = data.clock.get();
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

use dispatch::SystemInfo;
use handlers::OnCloseHandler;
use systems::TimerHandler;

impl SystemInfo for PingTimerHandler {
	type Dependencies = (OnCloseHandler, TimerHandler, SendLogin);

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

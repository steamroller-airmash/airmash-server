use specs::*;

use component::flag::IsPlayer;
use component::time::*;
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
	frame: Read<'a, ThisFrame>,
	conns: SendToAll<'a>,
	clock: ReadClock<'a>,

	is_player: ReadStorage<'a, IsPlayer>,
	pingdata: WriteStorage<'a, PingData>,
	associated: ReadStorage<'a, AssociatedConnection>,
}

impl PingTimerHandler {
	pub fn new() -> Self {
		Self {
			lastping: Instant::now(),
		}
	}
}

impl<'a> System<'a> for PingTimerHandler {
	type SystemData = PingTimerHandlerData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		if data.frame.0 < self.lastping + Duration::from_secs(5) {
			return;
		}
		self.lastping = data.frame.0;

		let now = Instant::now();
		let clock = data.clock.get();

		let ref mut pingdata = data.pingdata;
		let ref associated = data.associated;
		let ref conns = data.conns;

		(pingdata, associated, data.is_player.mask())
			.join()
			.for_each(|(pingdata, assoc, ..)| {
				let ping = pingdata.new_ping(now);

				conns.send_to(
					assoc.0,
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

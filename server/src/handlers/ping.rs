use specs::prelude::*;

use crate::component::flag::IsPlayer;
use crate::component::time::*;
use crate::types::systemdata::{Connections, ReadClock};
use crate::types::*;

use std::time::{Duration, Instant};

use crate::handlers::OnCloseHandler;
use crate::protocol::server::Ping as ServerPing;
use crate::systems::core::TimerHandler;
use crate::systems::handlers::game::on_join::SendLogin;

pub struct PingTimerHandler {
	lastping: Instant,
}

#[derive(SystemDataCustom)]
pub struct PingTimerHandlerData<'a> {
	frame: Read<'a, ThisFrame>,
	conns: Connections<'a>,
	clock: ReadClock<'a>,

	is_player: ReadStorage<'a, IsPlayer>,
	pingdata: WriteStorage<'a, PingData>,
	associated: ReadStorage<'a, AssociatedConnection>,
}

impl Default for PingTimerHandler {
	fn default() -> Self {
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

system_info! {
	impl SystemInfo for PingTimerHandler {
		type Dependencies = (OnCloseHandler, TimerHandler, SendLogin);
	}
}

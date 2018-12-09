use specs::world::EntitiesRes;
use specs::*;

use component::flag::IsPlayer;
use component::time::*;
use types::*;

use std::time::{Duration, Instant};

use protocol::server::Ping as ServerPing;
use systems::handlers::game::on_join::SendLogin;

pub struct PingTimerHandler {
	lastping: Instant,
}

#[derive(SystemData)]
pub struct PingTimerHandlerData<'a> {
	lazyupdate: Read<'a, LazyUpdate>,
	start: Read<'a, StartTime>,
	frame: Read<'a, ThisFrame>,
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

	fn run(&mut self, data: Self::SystemData) {
		if data.frame.0 < self.lastping + Duration::from_secs(5) {
			return;
		}
		self.lastping = data.frame.0;

		let start_time = data.start.0;

		data.lazyupdate.exec_mut(move |world| {
			let ref conns = world.read_resource::<Connections>();
			let ref entities = world.read_resource::<EntitiesRes>();
			let ref is_player = world.read_storage::<IsPlayer>();
			let ref mut pingdata = world.write_storage::<PingData>();

			let now = Instant::now();
			let clock = (now - start_time).to_clock();

			(entities, pingdata, is_player)
				.join()
				.for_each(|(ent, pingdata, ..)| {
					let ping = pingdata.new_ping(now);

					conns.send_to_player(
						ent,
						ServerPing {
							clock,
							num: ping.idx,
						},
					)
				})
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

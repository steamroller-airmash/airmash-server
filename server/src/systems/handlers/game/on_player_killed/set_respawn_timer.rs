
use specs::*;

use types::*;
use SystemInfo;

use std::any::Any;
use std::time::Duration;

use systems::missile::MissileHit;
use consts::timer::RESPAWN_TIME;
use component::event::*;
use component::channel::*;

pub struct SetRespawnTimer {
	reader: Option<OnPlayerKilledReader>
}

#[derive(SystemData)]
pub struct SetRespawnTimerData<'a> {
	pub channel: Read<'a, OnPlayerKilled>,
	pub future: ReadExpect<'a, FutureDispatcher>,
}

impl<'a> System<'a> for SetRespawnTimer {
	type SystemData = SetRespawnTimerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerKilled>().register_reader()
		);
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			info!("{:?}", evt);

			let player = evt.player;

			data.future.run_delayed(
				Duration::from_secs(2),
				move |instant| {
					info!("Timer Complete");

					Some(TimerEvent {
						ty: *RESPAWN_TIME,
						instant,
						data: Some(Box::new(player))
					})
				}
			);
		}
	}
}

impl SystemInfo for SetRespawnTimer {
	type Dependencies = MissileHit;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new(_: Box<Any>) -> Self {
		Self { reader: None }
	}
}

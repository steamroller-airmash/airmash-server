use specs::*;

use types::*;
use SystemInfo;

use std::time::Duration;

use component::channel::*;
use component::event::*;
use consts::timer::RESPAWN_TIME;
use systems::handlers::game::on_player_hit::AllPlayerHitSystems;
use systems::missile::MissileHit;

pub struct SetRespawnTimer {
	reader: Option<OnPlayerKilledReader>,
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

		self.reader = Some(res.fetch_mut::<OnPlayerKilled>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = evt.player;

			data.future
				.run_delayed(Duration::from_secs(2), move |instant| {
					Some(TimerEvent {
						ty: *RESPAWN_TIME,
						instant,
						data: Some(Box::new(player)),
					})
				});
		}
	}
}

impl SystemInfo for SetRespawnTimer {
	type Dependencies = (MissileHit, AllPlayerHitSystems);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

use server::*;
use specs::*;

use component::*;
use consts::*;
use systems::on_flag::CheckWin;

use server::component::event::TimerEvent;
use server::types::FutureDispatcher;

use std::time::Duration;

#[derive(Default)]
pub struct SetupMessages {
	reader: Option<OnGameWinReader>,
}

#[derive(SystemData)]
pub struct SetupMessagesData<'a> {
	channel: Read<'a, OnGameWin>,
	config: Write<'a, Config>,
	future: ReadExpect<'a, FutureDispatcher>,
}

impl<'a> System<'a> for SetupMessages {
	type SystemData = SetupMessagesData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameWin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			let config = &mut *data.config;
			let old_config = config.clone();

			let iter = config
				.mobs
				.0
				.iter_mut()
				.filter_map(|x| x.1.missile.as_mut());

			for missile in iter {
				missile.damage = Health::new(0.0);
			}

			data.future
				.run_delayed(Duration::from_secs(85), |inst| TimerEvent {
					ty: *RESTORE_CONFIG,
					instant: inst,
					data: Some(Box::new(old_config)),
				});
		}
	}
}

impl SystemInfo for SetupMessages {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

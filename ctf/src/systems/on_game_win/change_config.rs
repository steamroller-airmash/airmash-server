use server::*;
use specs::*;

use component::*;
use consts::*;
use systems::on_flag::CheckWin;

use server::component::event::TimerEvent;
use server::types::FutureDispatcher;

use std::time::Duration;

/// Set missile damage to 0 in the config
/// and dispatch a timer event to trigger
/// a config reset.
///
/// This system is a bit bizarre since it
/// writes to config, which isn't a normal
/// thing for a system to do, but I think
/// it's OK in this case since it cleanly
/// acheives the required functionality.
#[derive(Default)]
pub struct ChangeConfig {
	reader: Option<OnGameWinReader>,
}

#[derive(SystemData)]
pub struct ChangeConfigData<'a> {
	channel: Read<'a, OnGameWin>,
	config: Write<'a, Config>,
	future: ReadExpect<'a, FutureDispatcher>,
}

impl<'a> System<'a> for ChangeConfig {
	type SystemData = ChangeConfigData<'a>;

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

impl SystemInfo for ChangeConfig {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

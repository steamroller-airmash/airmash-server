use crate::server::*;
use specs::*;

use crate::component::*;
use crate::config::GAME_RESET_TIME;
use crate::consts::*;
use crate::systems::on_flag::CheckWin;

use crate::server::component::event::TimerEvent;
use crate::server::types::FutureDispatcher;
use crate::server::utils::*;

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
pub struct ChangeConfig;

#[derive(SystemData)]
pub struct ChangeConfigData<'a> {
	config: Write<'a, Config>,
	future: ReadExpect<'a, FutureDispatcher>,
}

impl EventHandlerTypeProvider for ChangeConfig {
	type Event = GameWinEvent;
}

impl<'a> EventHandler<'a> for ChangeConfig {
	type SystemData = ChangeConfigData<'a>;

	fn on_event(&mut self, _: &GameWinEvent, data: &mut Self::SystemData) {
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
			.run_delayed(*GAME_RESET_TIME, |inst| TimerEvent {
				ty: *RESTORE_CONFIG,
				instant: inst,
				data: Some(Box::new(old_config)),
			});
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

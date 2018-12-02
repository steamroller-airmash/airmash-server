use specs::*;

use server::component::event::*;
use server::utils::*;
use server::*;

use consts::*;

#[derive(Default)]
pub struct RestoreConfig;

#[derive(SystemData)]
pub struct RestoreConfigData<'a> {
	config: Write<'a, Config>,
}

impl EventHandlerTypeProvider for RestoreConfig {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for RestoreConfig {
	type SystemData = RestoreConfigData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *RESTORE_CONFIG {
			return;
		}

		let config = match evt.data {
			Some(ref dat) => match (*dat).downcast_ref::<Config>() {
				Some(val) => val,
				None => {
					error!("Unable to downcast TimerEvent data to Config!");
					return;
				}
			},
			None => return,
		};

		// This is costly, but it only happens once per
		// match so it should be ok
		*data.config = config.clone();
	}
}

impl SystemInfo for RestoreConfig {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

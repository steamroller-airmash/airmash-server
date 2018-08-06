use specs::*;

use server::component::channel::*;
use server::*;

use consts::*;

#[derive(Default)]
pub struct RestoreConfig {
	reader: Option<OnTimerEventReader>,
}

#[derive(SystemData)]
pub struct RestoreConfigData<'a> {
	channel: Read<'a, OnTimerEvent>,
	config: Write<'a, Config>,
}

impl<'a> System<'a> for RestoreConfig {
	type SystemData = RestoreConfigData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnTimerEvent>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if evt.ty != *RESTORE_CONFIG {
				continue;
			}

			let config = match evt.data {
				Some(ref dat) => match (*dat).downcast_ref::<Config>() {
					Some(val) => val,
					None => {
						error!("Unable to downcast TimerEvent data to Config!");
						continue;
					}
				},
				None => continue,
			};

			// This is costly, but it only happens once per
			// match so it should be ok
			*data.config = config.clone()
		}
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

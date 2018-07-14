use specs::*;
use types::*;

use SystemInfo;

use component::channel::*;
use component::event::TimerEvent;
use consts::timer::*;

use std::time::Duration;

pub struct SetUnthrottleTimer {
	reader: Option<OnPlayerThrottledReader>,
}

#[derive(SystemData)]
pub struct SetUnthrottleTimerData<'a> {
	channel: Read<'a, OnPlayerThrottled>,

	future: ReadExpect<'a, FutureDispatcher>,
}

impl<'a> System<'a> for SetUnthrottleTimer {
	type SystemData = SetUnthrottleTimerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerThrottled>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let evt = evt.clone();
			data.future
				.run_delayed(Duration::from_secs(5), move |inst| {
					Some(TimerEvent {
						ty: *UNTHROTTLE_TIME,
						instant: inst,
						data: Some(Box::new(evt.clone())),
					})
				});
		}
	}
}

impl SystemInfo for SetUnthrottleTimer {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

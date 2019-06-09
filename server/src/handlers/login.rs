use specs::*;

use component::channel::*;
use component::event::TimerEvent;
use consts::timer::*;
use types::*;

use std::sync::mpsc::*;
use std::time::Instant;

pub struct LoginHandler {
	reader: Option<OnLoginReader>,
	channel: Option<Sender<TimerEvent>>,
}

impl LoginHandler {
	pub fn new() -> Self {
		Self {
			reader: None,
			channel: None,
		}
	}
}

impl<'a> System<'a> for LoginHandler {
	type SystemData = Read<'a, OnLogin>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnLogin>().register_reader());
		self.channel = Some(res.fetch_mut::<FutureDispatcher>().get_channel());
	}

	fn run(&mut self, channel: Self::SystemData) {
		for evt in channel.read(self.reader.as_mut().unwrap()).cloned() {
			let channel = self.channel.as_ref().unwrap().clone();

			let event = TimerEvent {
				ty: *LOGIN_PASSED,
				instant: Instant::now(),
				data: Some(Box::new(evt)),
			};

			channel.send(event).unwrap();
		}
	}
}

use dispatch::SystemInfo;
use handlers::{OnCloseHandler, OnOpenHandler};

impl SystemInfo for LoginHandler {
	type Dependencies = (OnOpenHandler, OnCloseHandler);

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

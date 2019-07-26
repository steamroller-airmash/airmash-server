use specs::*;

use crate::component::channel::*;
use crate::component::event::TimerEvent;
use crate::consts::timer::*;
use crate::types::*;

use std::sync::mpsc::*;
use std::time::Instant;

#[derive(Default)]
pub struct LoginHandler {
	reader: Option<OnLoginReader>,
	channel: Option<Sender<TimerEvent>>,
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

system_info! {
	impl SystemInfo for LoginHandler {
		type Dependencies = (
			super::OnOpenHandler,
			super::OnCloseHandler
		);
	}
}

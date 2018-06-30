use std::any::Any;
use std::mem;
use std::sync::mpsc::{channel, Receiver};

use specs::*;

use component::channel::OnTimerEvent;
use component::event::*;
use dispatch::SystemInfo;

/// Forwards out-of-band timer events
/// into an EventChannel that can be
/// accessed by other systems.
pub struct TimerHandler {
	channel: Receiver<TimerEvent>,
}

impl TimerHandler {
	pub fn new(channel: Receiver<TimerEvent>) -> Self {
		Self { channel }
	}
}

#[derive(SystemData)]
pub struct TimerHandlerData<'a> {
	pub channel: Write<'a, OnTimerEvent>,
}

impl<'a> System<'a> for TimerHandler {
	type SystemData = TimerHandlerData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		while let Ok(evt) = self.channel.try_recv() {
			data.channel.single_write(evt);
		}
	}
}

impl SystemInfo for TimerHandler {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		unimplemented!();
	}

	fn new_args(mut a: Box<Any>) -> Self {
		let r = a.downcast_mut::<Receiver<TimerEvent>>().unwrap();
		// Replace the channel within the box with a
		// dummy one, which will be dropped immediately
		// anyway
		Self::new(mem::replace(r, channel().1))
	}
}

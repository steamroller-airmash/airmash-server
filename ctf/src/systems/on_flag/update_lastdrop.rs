use specs::*;

use component::*;

use server::component::time::ThisFrame;
use server::utils::*;
use server::*;

#[derive(Default)]
pub struct UpdateLastDrop;

#[derive(SystemData)]
pub struct UpdateLastDropData<'a> {
	lastdrop: WriteStorage<'a, LastDrop>,
	this_frame: Read<'a, ThisFrame>,
}

impl EventHandlerTypeProvider for UpdateLastDrop {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for UpdateLastDrop {
	type SystemData = UpdateLastDropData<'a>;

	fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
		let player = match evt.ty {
			FlagEventType::Capture => None,
			FlagEventType::Drop => evt.player,
			FlagEventType::Return => None,
			_ => return,
		};

		let lastdrop = try_get!(evt.flag, mut data.lastdrop);

		*lastdrop = LastDrop {
			player: player,
			time: data.this_frame.0,
		};
	}
}

impl SystemInfo for UpdateLastDrop {
	// It doesn't matter too much when we handle this
	// it can happen the next frame
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

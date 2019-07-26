use specs::*;

use crate::component::*;
use crate::systems::on_flag::AllFlagSystems;

use crate::server::utils::{EventHandler, EventHandlerTypeProvider};
use crate::server::SystemInfo;

#[derive(Default)]
pub struct ResetFlags;

#[derive(SystemData)]
pub struct ResetFlagsData<'a> {
	channel: Write<'a, OnFlag>,
	flags: ReadExpect<'a, Flags>,
}

impl EventHandlerTypeProvider for ResetFlags {
	type Event = GameWinEvent;
}

impl<'a> EventHandler<'a> for ResetFlags {
	type SystemData = ResetFlagsData<'a>;

	fn on_event(&mut self, _: &GameWinEvent, data: &mut Self::SystemData) {
		data.channel.single_write(FlagEvent {
			flag: data.flags.blue,
			player: None,
			ty: FlagEventType::Return,
		});
		data.channel.single_write(FlagEvent {
			flag: data.flags.red,
			player: None,
			ty: FlagEventType::Return,
		});
	}
}

impl SystemInfo for ResetFlags {
	type Dependencies = (super::SetGameActive, AllFlagSystems);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

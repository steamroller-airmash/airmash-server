use specs::prelude::*;

use crate::{
	component::{channel::*, collection::PlayerNames, event::PlayerLeave},
	systems::missile::MissileHit,
	utils::{EventHandler, EventHandlerTypeProvider},
};

#[derive(Default)]
pub struct FreeName;

#[derive(SystemDataCustom)]
pub struct FreeNameData<'a> {
	pub channel: Read<'a, OnPlayerLeave>,
	pub player_names: Write<'a, PlayerNames>,
}

impl EventHandlerTypeProvider for FreeName {
	type Event = PlayerLeave;
}

impl<'a> EventHandler<'a> for FreeName {
	type SystemData = FreeNameData<'a>;

	fn on_event(&mut self, evt: &PlayerLeave, data: &mut Self::SystemData) {
		data.player_names.0.remove_by_value(&evt.0);
	}
}

system_info! {
	impl SystemInfo for FreeName {
		type Dependencies = (MissileHit, super::KnownEventSources);
	}
}

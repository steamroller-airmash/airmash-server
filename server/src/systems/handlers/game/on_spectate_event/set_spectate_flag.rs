use specs::prelude::*;

use crate::component::event::PlayerSpectate;
use crate::component::flag::IsSpectating;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SetSpectateFlag;

#[derive(SystemData)]
pub struct SetSpectateFlagData<'a> {
	is_spec: WriteStorage<'a, IsSpectating>,
}

impl EventHandlerTypeProvider for SetSpectateFlag {
	type Event = PlayerSpectate;
}

impl<'a> EventHandler<'a> for SetSpectateFlag {
	type SystemData = SetSpectateFlagData<'a>;

	fn on_event(&mut self, evt: &PlayerSpectate, data: &mut Self::SystemData) {
		data.is_spec.insert(evt.player, IsSpectating).unwrap();
	}
}

system_info! {
	impl SystemInfo for SetSpectateFlag {
		type Dependencies = super::KnownEventSources;
	}
}

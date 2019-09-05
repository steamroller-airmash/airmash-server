use specs::prelude::*;

use crate::component::event::PlayerSpectate;
use crate::component::reference::PlayerRef;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SetSpectateTarget;

#[derive(SystemDataCustom)]
pub struct SetSpectateTargetData<'a> {
	spec_tgt: WriteStorage<'a, PlayerRef>,
}

impl EventHandlerTypeProvider for SetSpectateTarget {
	type Event = PlayerSpectate;
}

impl<'a> EventHandler<'a> for SetSpectateTarget {
	type SystemData = SetSpectateTargetData<'a>;

	fn on_event(&mut self, evt: &PlayerSpectate, data: &mut Self::SystemData) {
		let target = match evt.target {
			Some(ent) => ent,
			None => evt.player,
		};

		data.spec_tgt.insert(evt.player, PlayerRef(target)).unwrap();
	}
}

system_info! {
	impl SystemInfo for SetSpectateTarget {
		type Dependencies = super::KnownEventSources;
	}
}

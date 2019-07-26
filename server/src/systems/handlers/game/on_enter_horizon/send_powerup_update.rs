use specs::*;

use crate::component::event::*;
use crate::protocol::server::MobUpdateStationary;
use crate::types::systemdata::*;
use crate::types::*;
use crate::utils::*;

/// Sends [`MobUpdateStationary`](protocol::server::MobUpdateStationary)
/// when a powerup comes within viewing range of a player.
#[derive(Default)]
pub struct SendPowerupUpdate;

#[derive(SystemData)]
pub struct SendPowerupUpdateData<'a> {
	conns: SendToPlayer<'a>,

	pos: ReadStorage<'a, Position>,
	mob: ReadStorage<'a, Mob>,
}

impl EventHandlerTypeProvider for SendPowerupUpdate {
	type Event = EnterHorizon;
}

impl<'a> EventHandler<'a> for SendPowerupUpdate {
	type SystemData = SendPowerupUpdateData<'a>;

	fn on_event(&mut self, evt: &EnterHorizon, data: &mut Self::SystemData) {
		if evt.entered_ty != EntityType::Powerup {
			return;
		}

		let pos = *try_get!(evt.entered, data.pos);
		let mob = *try_get!(evt.entered, data.mob);

		data.conns.send_to_player(
			evt.player,
			MobUpdateStationary {
				id: evt.entered.into(),
				ty: mob,
				pos,
			},
		);
	}
}

system_info! {
	impl SystemInfo for SendPowerupUpdate {
		type Dependencies = super::KnownEventSources;
	}
}

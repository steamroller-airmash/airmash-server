use specs::*;

use crate::component::event::PlayerRespawn as EvtPlayerRespawn;
use crate::component::flag::*;
use crate::types::systemdata::SendToVisible;
use crate::types::*;
use crate::SystemInfo;

use crate::protocol::server::PlayerRespawn;
use crate::protocol::Upgrades as ProtocolUpgrades;

use crate::utils::{EventHandler, EventHandlerTypeProvider};

use crate::systems::handlers::command::AllCommandHandlers;
use crate::systems::handlers::game::on_join::AllJoinHandlers;
use crate::systems::handlers::game::on_player_respawn::SetTraits;

/// Send a [`PlayerRespawn`] packet to
/// all visible players if the target
/// player is not currently spectating.
#[derive(Default)]
pub struct SendPlayerRespawn;

#[derive(SystemData)]
pub struct SendPlayerRespawnData<'a> {
	entities: Entities<'a>,
	conns: SendToVisible<'a>,

	is_spec: ReadStorage<'a, IsSpectating>,
	pos: ReadStorage<'a, Position>,
	rot: ReadStorage<'a, Rotation>,
}

impl EventHandlerTypeProvider for SendPlayerRespawn {
	type Event = EvtPlayerRespawn;
}

impl<'a> EventHandler<'a> for SendPlayerRespawn {
	type SystemData = SendPlayerRespawnData<'a>;

	fn on_event(&mut self, evt: &EvtPlayerRespawn, data: &mut Self::SystemData) {
		if !data.entities.is_alive(evt.player) {
			return;
		}

		if data.is_spec.get(evt.player).is_some() {
			return;
		}

		let player = evt.player;
		let pos = *try_get!(player, data.pos);
		let rot = *try_get!(player, data.rot);

		data.conns.send_to_visible(
			pos,
			PlayerRespawn {
				id: player.into(),
				pos: pos,
				rot: rot,
				upgrades: ProtocolUpgrades::default(),
			},
		);
	}
}

impl SystemInfo for SendPlayerRespawn {
	type Dependencies = (AllJoinHandlers, SetTraits, AllCommandHandlers);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

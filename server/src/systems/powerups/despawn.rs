use specs::prelude::*;

use crate::{
	component::{
		channel::OnPowerupDespawn,
		event::{PlayerPowerupCollision, PowerupDespawnEvent},
		flag::IsPlayer,
	},
	types::{collision::Collision, Mob, Position},
	utils::{EventHandler, EventHandlerTypeProvider},
};

#[derive(Default)]
pub struct Despawn;

#[derive(SystemDataCustom)]
pub struct DespawnData<'a> {
	channel: Write<'a, OnPowerupDespawn>,

	is_player: ReadStorage<'a, IsPlayer>,
	mob: ReadStorage<'a, Mob>,
	pos: ReadStorage<'a, Position>,
}

impl EventHandlerTypeProvider for Despawn {
	type Event = PlayerPowerupCollision;
}

impl<'a> EventHandler<'a> for Despawn {
	type SystemData = DespawnData<'a>;

	fn on_event(&mut self, evt: &Self::Event, data: &mut DespawnData) {
		let Collision(c1, c2) = evt.0;

		let (player, powerup) = match data.is_player.get(c1.ent) {
			Some(_) => (c1, c2),
			None => (c2, c1),
		};

		data.channel.single_write(PowerupDespawnEvent {
			mob: powerup.ent,
			ty: *try_get!(powerup.ent, data.mob),
			pos: *try_get!(powerup.ent, data.pos),
			player: Some(player.ent),
		});
	}
}

system_info! {
	impl SystemInfo for Despawn {
		type Dependencies = ();
	}
}

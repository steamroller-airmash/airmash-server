//! Event handlers that run when a powerup despawns

use crate::{
	component::{flag::IsZombie, powerup::*},
	consts::missile::ID_REUSE_TIME,
	protocol::{server::MobDespawn, DespawnType},
	types::{systemdata::Connections, Mob, Position, TaskSpawner, Velocity},
	utils::HistoricalStorageExt,
};

use specs::prelude::*;

/// Remove the components from powerup and set it
/// up for deletion after `ID_REUSE_TIME` has passed.
///
/// Also inserts the `IsZombie` flag for debugging.
#[event_handler(name = Cleanup)]
fn cleanup<'a>(
	evt: &PowerupDespawn,

	entities: &Entities<'a>,
	tasks: &ReadExpect<'a, TaskSpawner>,
	lazy: &Read<'a, LazyUpdate>,

	is_zombie: &mut WriteStorage<'a, IsZombie>,
) {
	if !entities.is_alive(evt.mob) {
		return;
	}

	if is_zombie.mask().contains(evt.mob.id()) {
		return;
	}

	is_zombie
		.insert_with_history(evt.mob, IsZombie::from_sys(&Cleanup))
		.unwrap();

	tasks.spawn(crate::task::delayed_delete(
		tasks.task_data(),
		evt.mob,
		ID_REUSE_TIME,
	));

	lazy.remove::<IsPowerup>(evt.mob);
	lazy.remove::<Mob>(evt.mob);
	lazy.remove::<Position>(evt.mob);
	lazy.remove::<Velocity>(evt.mob);
}

/// Send the `MobDespawn` packet.
#[event_handler(name=SendPacket)]
fn send_packet<'a>(evt: &PowerupDespawn, conns: &Connections<'a>) {
	let ty = match evt.player {
		Some(_) => DespawnType::Collided,
		None => DespawnType::LifetimeEnded,
	};

	conns.send_to_visible(
		evt.pos,
		MobDespawn {
			id: evt.mob.into(),
			ty,
		},
	);
}

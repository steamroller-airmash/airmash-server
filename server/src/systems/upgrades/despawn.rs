use specs::*;

use crate::component::channel::OnPowerupDespawn;
use crate::component::event::PowerupDespawnEvent;
use crate::component::time::{MobDespawnTime, ThisFrame};
use crate::types::*;

#[derive(Default)]
pub struct Despawn;

#[derive(SystemData)]
pub struct DespawnData<'a> {
	frame: Read<'a, ThisFrame>,
	entities: Entities<'a>,
	channel: Write<'a, OnPowerupDespawn>,

	mob: ReadStorage<'a, Mob>,
	despawn: ReadStorage<'a, MobDespawnTime>,
	pos: ReadStorage<'a, Position>,
}

impl<'a> System<'a> for Despawn {
	type SystemData = DespawnData<'a>;

	fn run(&mut self, mut data: DespawnData<'a>) {
		let frame = data.frame.0;

		let it = (&*data.entities, &data.despawn, &data.mob, &data.pos)
			.join()
			.filter(|(_, despawn, ..)| despawn.0 < frame)
			.map(|(ent, _, mob, pos)| PowerupDespawnEvent {
				mob: ent,
				ty: *mob,
				pos: *pos,
				player: None,
			});

		for evt in it {
			data.entities.delete(evt.mob).unwrap();
			data.channel.single_write(evt);
		}
	}
}

system_info! {
	impl SystemInfo for Despawn {
		type Dependencies = ();
	}
}

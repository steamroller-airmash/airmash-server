use specs::*;

use types::collision::*;
use types::*;

use component::channel::*;
use component::event::*;
use component::flag::*;
use systems;
use utils::*;

#[derive(Default)]
pub struct PickupUpgrade;

#[derive(SystemData)]
pub struct PickupUpgradeData<'a> {
	upgrade_channel: Write<'a, OnPowerupPickup>,
	entities: Entities<'a>,

	mobs: ReadStorage<'a, Mob>,
	upgrades: WriteStorage<'a, Upgrades>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl EventHandlerTypeProvider for PickupUpgrade {
	type Event = PlayerPowerupCollision;
}

impl<'a> EventHandler<'a> for PickupUpgrade {
	type SystemData = PickupUpgradeData<'a>;

	fn on_event(&mut self, evt: &PlayerPowerupCollision, data: &mut Self::SystemData) {
		let Collision(c1, c2) = evt.0;

		let (player, upgrade) = match data.is_player.get(c1.ent) {
			Some(_) => (c1, c2),
			None => (c2, c1),
		};

		if !data.entities.is_alive(upgrade.ent) {
			return;
		}

		if *try_get!(upgrade.ent, data.mobs) != Mob::Upgrade {
			return;
		}

		try_get!(player.ent, mut data.upgrades).unused += 1;

		data.upgrade_channel.single_write(PowerupPickupEvent {
			pos: upgrade.pos,
			upgrade: upgrade.ent,
			player: player.ent,
		})
	}
}

system_info! {
	impl SystemInfo for PickupUpgrade {
		type Dependencies = systems::collision::PlayerPowerupCollisionSystem;
	}
}

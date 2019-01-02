use specs::*;

use types::collision::*;
use types::*;

use component::channel::*;
use component::event::*;
use component::flag::*;
use systems;
use utils::*;
use protocol::PowerupType;

use std::time::Instant;

#[derive(Default)]
pub struct Pickup;

#[derive(SystemData)]
pub struct PickupData<'a> {
	upgrade_channel: Write<'a, OnPlayerPowerup>,
	entities: Entities<'a>,
	config: Read<'a, Config>,

	mobs: ReadStorage<'a, Mob>,
	powerups: WriteStorage<'a, Powerups>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl EventHandlerTypeProvider for Pickup {
	type Event = PlayerPowerupCollision;
}

impl<'a> EventHandler<'a> for Pickup {
	type SystemData = PickupData<'a>;

	fn on_event(&mut self, evt: &PlayerPowerupCollision, data: &mut Self::SystemData) {
		let Collision(c1, c2) = evt.0;

		let (player, upgrade) = match data.is_player.get(c1.ent) {
			Some(_) => (c1, c2),
			None => (c2, c1),
		};

		if !data.entities.is_alive(upgrade.ent) {
			return;
		}

		let (duration, ty) = match *try_get!(upgrade.ent, data.mobs) {
			Mob::Shield => (data.config.shield_duration, PowerupType::Shield),
			Mob::Inferno => (data.config.inferno_duration, PowerupType::Inferno),
			_ => return
		};

		data.powerups.insert(
			player.ent,
			Powerups {
				end_time: Instant::now() + duration,
				ty
			}
		).unwrap();

		data.entities.delete(upgrade.ent).unwrap();

		data.upgrade_channel.single_write(PlayerPowerup {
			player: player.ent,
			duration,
			ty
		})
	}
}

system_info! {
	impl SystemInfo for Pickup {
		type Dependencies = systems::collision::PlayerPowerupCollisionSystem;
	}
}

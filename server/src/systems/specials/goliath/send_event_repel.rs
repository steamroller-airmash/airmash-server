use crate::types::systemdata::*;
use crate::types::*;
use specs::*;

use crate::SystemInfo;

use crate::component::event::PlayerRepel;
use crate::component::flag::{IsMissile, IsPlayer};
use crate::component::reference::PlayerRef;
use crate::systems::specials::config::*;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

use crate::protocol::server::{EventRepel, EventRepelMob, EventRepelPlayer};

/// Send [`EventRepel`] when a goliath uses it's special.
///
/// This system also sets the position, speed, velocity,
/// team and owner for players and mobs that
/// are caught in the deflection.
#[derive(Default)]
pub struct SendEventRepel;

#[derive(SystemData)]
pub struct SendEventRepelData<'a> {
	conns: SendToVisible<'a>,
	config: Read<'a, Config>,
	entities: Entities<'a>,
	clock: ReadClock<'a>,

	pos: ReadStorage<'a, Position>,
	plane: ReadStorage<'a, Plane>,
	mob: ReadStorage<'a, Mob>,
	team: WriteStorage<'a, Team>,
	vel: WriteStorage<'a, Velocity>,
	rot: WriteStorage<'a, Rotation>,
	health: ReadStorage<'a, Health>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
	owner: WriteStorage<'a, PlayerRef>,
	keystate: ReadStorage<'a, KeyState>,
	is_alive: IsAlive<'a>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_missile: ReadStorage<'a, IsMissile>,
}

impl EventHandlerTypeProvider for SendEventRepel {
	type Event = PlayerRepel;
}

impl<'a> EventHandler<'a> for SendEventRepel {
	type SystemData = SendEventRepelData<'a>;

	fn on_event(&mut self, evt: &PlayerRepel, data: &mut Self::SystemData) {
		let player_r2 = *GOLIATH_SPECIAL_RADIUS_PLAYER * *GOLIATH_SPECIAL_RADIUS_PLAYER;
		let missile_r2 = *GOLIATH_SPECIAL_RADIUS_MISSILE * *GOLIATH_SPECIAL_RADIUS_MISSILE;

		let pos = *try_get!(evt.player, data.pos);
		let team = *try_get!(evt.player, data.team);

		let hit_players = (
			&*data.entities,
			&data.pos,
			&data.team,
			&data.is_player,
			data.is_alive.mask(),
		)
			.join()
			.filter(|(ent, ..)| *ent != evt.player)
			.filter(|(_, _, &target_team, ..)| target_team != team)
			.filter_map(|(ent, player_pos, ..)| {
				let dist2 = (*player_pos - pos).length2();

				if dist2 < player_r2 {
					Some((ent, *player_pos))
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let hit_missiles = (&*data.entities, &data.pos, &data.team, &data.is_missile)
			.join()
			.filter(|(_, _, missile_team, ..)| **missile_team != team)
			.filter_map(|(ent, missile_pos, ..)| {
				let dist2 = (*missile_pos - pos).length2();

				if dist2 < missile_r2 {
					Some((ent, *missile_pos))
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		for (player, player_pos) in hit_players.iter() {
			let dir = (*player_pos - pos).normalized();

			*try_get!(*player, mut data.vel) = dir * *GOLIATH_SPECIAL_REFLECT_SPEED;
		}

		for (missile, missile_pos) in hit_missiles.iter() {
			let dir = (*missile_pos - pos).normalized();

			*try_get!(*missile, mut data.vel) = dir * *GOLIATH_SPECIAL_REFLECT_SPEED;
			// Change the team of the missile to reflect
			// that it's now owned by the player that
			// reflected it
			*try_get!(*missile, mut data.team) = team;
			// Change the owner of the missile now that
			// it's been reflected
			*try_get!(*missile, mut data.owner) = PlayerRef(evt.player);
		}

		let players = hit_players
			.into_iter()
			// The largest a serialized SmallArray can be is 255 elements
			.take(255)
			.filter_map(|(player, player_pos)| {
				let plane = *log_none!(player, data.plane)?;
				let keystate = log_none!(player, data.keystate)?.to_server(&plane);
				let ref info = data.config.planes[plane];

				EventRepelPlayer {
					id: player.into(),
					keystate,
					health: *log_none!(player, data.health)?,
					health_regen: info.health_regen,
					energy: *log_none!(player, data.energy)?,
					energy_regen: *log_none!(player, data.energy_regen)?,
					pos: player_pos,
					rot: *log_none!(player, data.rot)?,
					speed: *log_none!(player, data.vel)?,
				}
				.into()
			})
			.collect::<Vec<_>>();

		let mobs = hit_missiles
			.into_iter()
			// The largest a serialized SmallArray can be is 255 elements
			.take(255)
			.filter_map(|(missile, missile_pos)| {
				let mob = *log_none!(missile, data.mob)?;
				let ref info = data.config.mobs[mob].missile.unwrap();
				let dir = (missile_pos - pos).normalized();

				EventRepelMob {
					id: missile.into(),
					pos: missile_pos,
					accel: dir * info.accel,
					speed: *log_none!(missile, data.vel)?,
					max_speed: info.max_speed,
					ty: mob,
				}
				.into()
			})
			.collect::<Vec<_>>();

		let packet = EventRepel {
			clock: data.clock.get(),
			id: evt.player.into(),
			energy: *try_get!(evt.player, data.energy),
			energy_regen: *try_get!(evt.player, data.energy_regen),
			rot: *try_get!(evt.player, data.rot),
			speed: *try_get!(evt.player, data.vel),
			pos: pos,
			mobs,
			players,
		};

		data.conns.send_to_visible(pos, packet);
	}
}

impl SystemInfo for SendEventRepel {
	type Dependencies = super::GoliathRepel;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

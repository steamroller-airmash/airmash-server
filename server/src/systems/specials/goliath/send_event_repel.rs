use specs::*;
use types::systemdata::*;
use types::*;

use SystemInfo;

use component::channel::{OnPlayerRepel, OnPlayerRepelReader};
use component::flag::{IsMissile, IsPlayer};
use component::reference::PlayerRef;
use systems::specials::config::*;

use protocol::server::{EventRepel, EventRepelMob, EventRepelPlayer};

/// Send [`EventRepel`] when a goliath uses it's special.
///
/// This system also position, speed, velocity,
/// team and owner for players and mobs that
/// are caught in the deflection.
#[derive(Default)]
pub struct SendEventRepel {
	reader: Option<OnPlayerRepelReader>,
}

#[derive(SystemData)]
pub struct SendEventRepelData<'a> {
	conns: Read<'a, Connections>,
	channel: Read<'a, OnPlayerRepel>,
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

impl SendEventRepel {}

impl<'a> System<'a> for SendEventRepel {
	type SystemData = SendEventRepelData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerRepel>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let player_r2 = *GOLIATH_SPECIAL_RADIUS_PLAYER * *GOLIATH_SPECIAL_RADIUS_PLAYER;
		let missile_r2 = *GOLIATH_SPECIAL_RADIUS_MISSILE * *GOLIATH_SPECIAL_RADIUS_MISSILE;

		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let pos = *data.pos.get(evt.player).unwrap();
			let team = *data.team.get(evt.player).unwrap();

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
				}).collect::<Vec<_>>();

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
				}).collect::<Vec<_>>();

			for (player, player_pos) in hit_players.iter() {
				let dir = (*player_pos - pos).normalized();

				*data.vel.get_mut(*player).unwrap() = dir * *GOLIATH_SPECIAL_REFLECT_SPEED;
			}

			for (missile, missile_pos) in hit_missiles.iter() {
				let dir = (*missile_pos - pos).normalized();

				*data.vel.get_mut(*missile).unwrap() = dir * *GOLIATH_SPECIAL_REFLECT_SPEED;
				// Change the team of the missile to reflect
				// that it's now owned by the player that
				// reflected it
				*data.team.get_mut(*missile).unwrap() = team;
				// Change the owner of the missile now that
				// it's been reflected
				*data.owner.get_mut(*missile).unwrap() = PlayerRef(evt.player);
			}

			let players = hit_players
				.into_iter()
				// The largest a serialized SmallArray can be is 255 elements
				.take(255)
				.map(|(player, player_pos)| {
					let plane = *data.plane.get(player).unwrap();
					let keystate = data.keystate.get(player).unwrap().to_server(&plane);
					let ref info = data.config.planes[plane];

					EventRepelPlayer {
						id: player.into(),
						keystate,
						health: *data.health.get(player).unwrap(),
						health_regen: info.health_regen,
						energy: *data.energy.get(player).unwrap(),
						energy_regen: *data.energy_regen.get(player).unwrap(),
						pos: player_pos,
						rot: *data.rot.get(player).unwrap(),
						speed: *data.vel.get(player).unwrap(),
					}
				})
				.collect::<Vec<_>>();

			let mobs = hit_missiles
				.into_iter()
				// The largest a serialized SmallArray can be is 255 elements
				.take(255)
				.map(|(missile, missile_pos)| {
					let mob = *data.mob.get(missile).unwrap();
					let ref info = data.config.mobs[mob].missile.unwrap();
					let dir = (missile_pos - pos).normalized();

					EventRepelMob {
						id: missile.into(),
						pos: missile_pos,
						accel: dir * info.accel,
						speed: *data.vel.get(missile).unwrap(),
						max_speed: info.max_speed,
						ty: mob,
					}
				})
				.collect::<Vec<_>>();

			let packet = EventRepel {
				clock: data.clock.get(),
				id: evt.player.into(),
				energy: *data.energy.get(evt.player).unwrap(),
				energy_regen: *data.energy_regen.get(evt.player).unwrap(),
				rot: *data.rot.get(evt.player).unwrap(),
				speed: *data.vel.get(evt.player).unwrap(),
				pos: pos,
				mobs,
				players,
			};

			data.conns.send_to_visible(evt.player, packet);
		}
	}
}

impl SystemInfo for SendEventRepel {
	type Dependencies = super::GoliathRepel;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

use specs::prelude::*;

use hashbrown::{HashMap, HashSet};

use std::iter::FromIterator;

// TODO: Properly figure out missing missiles
#[allow(unused_imports)]
use crate::component::{
	channel::{OnLeaveHorizon, OnMissileDespawn, OnPlayerLeave, OnPowerupDespawn},
	collision::{MissileGrid, PlayerGrid, PowerupGrid},
	event::{EntityType, LeaveHorizon, MissileDespawnType},
	flag::{IsMissile, IsPlayer, IsPowerup},
};
use crate::task::TaskData;
use crate::types::{
	collision::{Grid, HitCircle},
	systemdata::IsAlive,
};
use crate::{Config, Distance, Mob, Position, Team};

use std::mem;

#[derive(SystemData)]
struct FillGridData<'a> {
	config: Read<'a, Config>,
	entities: Entities<'a>,

	player_grid: Write<'a, PlayerGrid>,
	missile_grid: Write<'a, MissileGrid>,
	powerup_grid: Write<'a, PowerupGrid>,

	leave_horizon: Write<'a, OnLeaveHorizon>,

	team: ReadStorage<'a, Team>,
	mob: ReadStorage<'a, Mob>,
	pos: ReadStorage<'a, Position>,

	is_missile: ReadStorage<'a, IsMissile>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_powerup: ReadStorage<'a, IsPowerup>,
	is_alive: IsAlive<'a>,
}

#[allow(unused)]
#[derive(SystemData)]
struct Channels<'a> {
	player_leave: Read<'a, OnPlayerLeave>,
	missile_despawn: Read<'a, OnMissileDespawn>,
	powerup_despawn: Read<'a, OnPowerupDespawn>,
}

pub async fn calculate_visibility(mut data: TaskData) {
	let mut spare_player_grid = Grid::new(vec![]);
	let mut spare_missile_grid = Grid::new(vec![]);
	let mut spare_powerup_grid = Grid::new(vec![]);
	let mut visibility_set: HashMap<_, HashSet<Entity>> = HashMap::default();

	let mut player_leave_reader =
		data.write_resource::<OnPlayerLeave, _, _>(|mut res| res.register_reader());
	let mut missile_despawn_reader =
		data.write_resource::<OnMissileDespawn, _, _>(|mut res| res.register_reader());
	let mut powerup_despawn_reader =
		data.write_resource::<OnPowerupDespawn, _, _>(|mut res| res.register_reader());

	loop {
		data.yield_frame().await;

		// Clear out entities that would disappear without an
		// EventLeaveHorizon packet being sent.
		data.world(|world| {
			let channels: Channels = world.system_data();

			let players = channels
				.player_leave
				.read(&mut player_leave_reader)
				.map(|evt| evt.0);
			let missiles = channels
				.missile_despawn
				.read(&mut missile_despawn_reader)
				// .filter(|evt| evt.ty != MissileDespawnType::LifetimeEnded)
				.map(|evt| evt.missile);
			let powerups = channels
				.powerup_despawn
				.read(&mut powerup_despawn_reader)
				.filter(|evt| evt.player.is_none())
				.map(|evt| evt.mob);

			let disappeared = players.chain(missiles).chain(powerups);

			for ent in disappeared {
				for (_, set) in visibility_set.iter_mut() {
					set.remove(&ent);
				}
			}
		});

		data.world(|world| {
			let mut data: FillGridData = world.system_data();

			let player_circles = (
				&*data.entities,
				&data.pos,
				&data.team,
				// Only players
				data.is_player.mask(),
				// Which are currently alive
				data.is_alive.mask(),
			)
				.join()
				.map(|(ent, pos, team, ..)| HitCircle {
					ent,
					pos: *pos,
					layer: team.0,
					rad: Distance::new(0.0),
				});
			spare_player_grid.rebuild_from(player_circles);

			let missile_circles = (
				&*data.entities,
				&data.pos,
				&data.team,
				data.is_missile.mask(),
			)
				.join()
				.map(|(ent, pos, team, ..)| HitCircle {
					ent,
					pos: *pos,
					layer: team.0,
					rad: Distance::new(0.0),
				});
			spare_missile_grid.rebuild_from(missile_circles);

			let powerup_circles = (&*data.entities, &data.mob, &data.pos, &data.team)
				.join()
				.filter(|(_, mob, ..)| match mob {
					Mob::Inferno | Mob::Shield | Mob::Upgrade => true,
					_ => false,
				})
				.map(|(ent, _, pos, team, ..)| HitCircle {
					ent,
					pos: *pos,
					layer: team.0,
					rad: Distance::new(0.0),
				});
			spare_powerup_grid.rebuild_from(powerup_circles);

			mem::swap(&mut spare_player_grid, &mut data.player_grid.0);
			mem::swap(&mut spare_missile_grid, &mut data.missile_grid.0);
			mem::swap(&mut spare_powerup_grid, &mut data.powerup_grid.0);

			let visible: HashMap<_, _> = (&*data.entities, &data.pos, data.is_player.mask())
				.join()
				.map(|(ent, pos, ..)| {
					let hc = HitCircle {
						pos: *pos,
						rad: data.config.view_radius,
						layer: 0,
						ent,
					};

					let visible_players = data.player_grid.rough_collide(hc);
					let visible_missiles = data.missile_grid.rough_collide(hc);
					let visible_powerups = data.powerup_grid.rough_collide(hc);

					let visible: HashSet<_> = HashSet::from_iter(
						visible_players
							.into_iter()
							.chain(visible_missiles.into_iter())
							.chain(visible_powerups.into_iter()),
					);

					(ent, visible)
				})
				.collect();

			for (player, new_visible) in &visible {
				let old_visible: &HashSet<Entity> = match visibility_set.get(player) {
					Some(x) => x,
					None => continue,
				};

				for &ent in old_visible {
					if !new_visible.contains(&ent) {
						let ty = match () {
							_ if data.is_player.get(ent).is_some() => EntityType::Player,
							_ if data.is_missile.get(ent).is_some() => EntityType::Missile,
							_ if data.is_powerup.get(ent).is_some() => EntityType::Powerup,
							_ => {
								error!(
									"Entity was not a powerup, missile, or player: {:#?}",
									world.debug_entity(ent)
								);
								continue;
							}
						};

						data.leave_horizon.single_write(LeaveHorizon {
							player: *player,
							left: ent,
							left_ty: ty,
						});
					}
				}
			}

			let _ = mem::replace(&mut visibility_set, visible);
		});
	}
}

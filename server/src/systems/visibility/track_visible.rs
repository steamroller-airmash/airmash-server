use hashbrown::{HashMap, HashSet};
use shrev::*;
use specs::*;

use component::collision::*;
use component::event::*;
use component::flag::*;
use types::collision::{Grid, HitCircle};
use types::{Config, Position};
use utils::MaybeInit;
use SystemInfo;

use systems;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct VisibleEntry {
	/// The type of the entity
	ty: EntityType,
	ent: Entity,
}

#[derive(Default)]
pub struct TrackVisible {
	visible: HashMap<Entity, HashSet<VisibleEntry>>,

	player_join: MaybeInit<ReaderId<PlayerJoin>>,
	player_leave: MaybeInit<ReaderId<PlayerLeave>>,
	player_fire: MaybeInit<ReaderId<MissileFire>>,
	player_despawn: MaybeInit<ReaderId<PlayerDespawn>>,
}

#[derive(SystemData)]
pub struct TrackVisibleData<'a> {
	entities: Entities<'a>,
	config: Read<'a, Config>,

	// FIXME: Add grid for upgrades
	players: Read<'a, PlayerGrid>,
	missiles: Read<'a, MissileGrid>,

	// Various event channels that will introduce/remove
	// visible players (the client already knows about these)
	player_join: Read<'a, EventChannel<PlayerJoin>>,
	player_leave: Read<'a, EventChannel<PlayerLeave>>,
	player_fire: Read<'a, EventChannel<MissileFire>>,
	player_despawn: Read<'a, EventChannel<PlayerDespawn>>,
	// TODO: Add upgrades
	leave_horizon: Write<'a, EventChannel<LeaveHorizon>>,
	enter_horizon: Write<'a, EventChannel<EnterHorizon>>,

	pos: ReadStorage<'a, Position>,
	is_player: ReadStorage<'a, IsPlayer>,
}

macro_rules! register_reader {
	($res:expr, $evt:ty) => {
		MaybeInit::new($res.fetch_mut::<EventChannel<$evt>>().register_reader())
	};
}

impl TrackVisible {
	/// Get rid of the entries for the players that
	/// have just left the game.
	fn remove_players_left<'a>(&mut self, data: &TrackVisibleData<'a>) {
		for evt in data.player_leave.read(&mut *self.player_leave) {
			self.visible.remove(&evt.0);

			for set in self.visible.values_mut() {
				set.remove(&VisibleEntry {
					ty: EntityType::Player,
					ent: evt.0,
				});
			}
		}
	}

	/// Add entries for players that just joined.
	fn add_players_join<'a>(&mut self, data: &TrackVisibleData<'a>) {
		for evt in data.player_join.read(&mut *self.player_join) {
			self.visible.insert(evt.id, HashSet::default());
		}
	}

	/// Add entries for missiles which have just been
	/// fired. Players don't need an update for these
	/// since they just got one.
	fn add_fired_missiles<'a>(&mut self, data: &TrackVisibleData<'a>) {
		for evt in data.player_fire.read(&mut *self.player_fire) {
			for missile in evt.missiles.iter().cloned() {
				let pos = match log_none!(missile, data.pos) {
					Some(x) => *x,
					None => continue,
				};

				let viewed = data.players.0.rough_collide(HitCircle {
					pos: pos,
					rad: data.config.view_radius.into(),
					ent: missile,
					layer: 0,
				});

				for player in viewed {
					let ref mut entry = self.visible.get_mut(&player).unwrap();

					entry.insert(VisibleEntry {
						ent: missile,
						ty: EntityType::Missile,
					});
				}
			}
		}
	}

	/// Remove player who have despawned (for various reasons).
	fn remove_players_despawned<'a>(&mut self, data: &TrackVisibleData<'a>) {
		for evt in data.player_despawn.read(&mut *self.player_despawn) {
			self.visible.remove(&evt.player);

			for set in self.visible.values_mut() {
				set.remove(&VisibleEntry {
					ent: evt.player,
					ty: EntityType::Player,
				});
			}
		}
	}

	fn rough_collide(
		pos: Position,
		ent: Entity,
		grid: &Grid,
		config: &Config,
	) -> impl Iterator<Item = VisibleEntry> {
		grid.rough_collide(HitCircle {
			pos: pos,
			ent: ent,
			rad: config.view_radius.into(),
			layer: 0,
		})
		.into_iter()
		.map(|x| VisibleEntry {
			ent: x,
			ty: EntityType::Player,
		})
	}

	fn send_events<'a>(&mut self, data: &mut TrackVisibleData<'a>) {
		let ref mut enter_horizon = data.enter_horizon;
		let ref mut leave_horizon = data.leave_horizon;

		let ref players = data.players;
		let ref missiles = data.missiles;
		let ref config = data.config;

		(&*data.entities, &data.pos, data.is_player.mask())
			.join()
			.for_each(|(ent, pos, ..)| {
				// TODO: Add support for upgrades
				let players = Self::rough_collide(*pos, ent, &players.0, &*config);
				let missiles = Self::rough_collide(*pos, ent, &missiles.0, &*config);

				let union: HashSet<_> = players.chain(missiles).collect();

				let old;
				if self.visible.contains_key(&ent) {
					old = self.visible.get_mut(&ent).unwrap();
				} else {
					error!("Visible was missing an entry for {:?}, creating one.", ent);
					self.visible.insert(ent, HashSet::default());
					old = self.visible.get_mut(&ent).unwrap();
				}

				{
					let added = HashSet::difference(&old, &union);
					let removed = HashSet::difference(&union, &old);

					for x in added {
						let evt = EnterHorizon {
							player: ent,
							entered: x.ent,
							entered_ty: x.ty,
						};

						enter_horizon.single_write(evt);
					}

					for x in removed {
						let evt = LeaveHorizon {
							player: ent,
							left: x.ent,
							left_ty: x.ty,
						};

						leave_horizon.single_write(evt);
					}
				}

				*old = union;
			});
	}
}

impl<'a> System<'a> for TrackVisible {
	type SystemData = TrackVisibleData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.player_join = register_reader!(res, PlayerJoin);
		self.player_leave = register_reader!(res, PlayerLeave);
		self.player_fire = register_reader!(res, MissileFire);
		self.player_despawn = register_reader!(res, PlayerDespawn);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		// TODO: Deal with upgrades
		self.add_players_join(&data);
		self.add_fired_missiles(&data);
		self.remove_players_despawned(&data);
		self.remove_players_left(&data);

		self.send_events(&mut data);
	}
}

impl SystemInfo for TrackVisible {
	type Dependencies = (
		systems::handlers::game::on_join::AllJoinHandlers,
		systems::handlers::game::on_missile_despawn::KnownEventSources,
		systems::handlers::game::on_player_despawn::KnownEventSources,
		systems::handlers::game::on_leave::KnownEventSources,
		systems::handlers::game::on_missile_fire::KnownEventSources,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

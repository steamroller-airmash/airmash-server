use specs::*;
use types::*;

use std::iter::Iterator;

use component::channel::OnMissileFire;
use component::event::MissileFire;
use component::flag::*;
use component::missile::MissileTrajectory;
use component::reference::PlayerRef;
use component::time::*;

use super::IsAlive;

pub struct MissileFireInfo {
	/// Starting offset of the missile, relative
	/// to the plane that is firing it. This will
	/// be rotated into the plane's frame of reference.
	pub pos_offset: Position,
	/// Direction that the missile will accelerate
	/// in, relative to the direction the plane
	/// is facing when it fires
	pub rot_offset: Rotation,
	/// Type of the missile
	pub ty: Mob,
}

#[derive(SystemData)]
pub struct FireMissiles<'a> {
	pub entities: Entities<'a>,
	pub this_frame: Read<'a, ThisFrame>,
	pub channel: Write<'a, OnMissileFire>,
	pub config: Read<'a, Config>,

	pub is_player: ReadStorage<'a, IsPlayer>,
	pub is_alive: IsAlive<'a>,
	pub upgrades: ReadStorage<'a, Upgrades>,

	pub team: WriteStorage<'a, Team>,
	pub pos: WriteStorage<'a, Position>,
	pub rot: ReadStorage<'a, Rotation>,
	pub vel: WriteStorage<'a, Velocity>,
	pub owner: WriteStorage<'a, PlayerRef>,
	pub mob: WriteStorage<'a, Mob>,
	pub is_missile: WriteStorage<'a, IsMissile>,
	pub spawn_time: WriteStorage<'a, MobSpawnTime>,
	pub missile_trajectory: WriteStorage<'a, MissileTrajectory>,
}

impl<'a> FireMissiles<'a> {
	pub fn fire_missiles(&mut self, owner: Entity, missiles: &[MissileFireInfo]) {
		self.is_player
			.get(owner)
			.expect("Entity firing a missile was not a player");

		if !self.is_alive.get(owner) {
			panic!("Entity firing a missile was not alive");
		}

		let rot = *self.rot.get(owner).unwrap();
		let vel = *self.vel.get(owner).unwrap();
		let pos = *self.pos.get(owner).unwrap();
		let team = *self.team.get(owner).unwrap();
		let upg_factor =
			self.config.upgrades.missile.factor[self.upgrades.get(owner).unwrap().missile as usize];
		let speed = vel.length();
		let spawn_time = MobSpawnTime(self.this_frame.0);

		let missiles = missiles
			.iter()
			.map(|info| {
				let rot = rot + info.rot_offset;
				// Subtract since airmash's coordinate system is flipped
				let pos = pos - info.pos_offset.rotate(rot);

				let missile = self.config.mobs[info.ty]
					.missile
					.expect("Mob was not a missile, you can only fire missiles!");

				// Rotate starting angle 90 degrees so that
				// it's inline with the plane. Change this
				// and missiles will shoot sideways
				let dir = Vector2::<f32>::new(rot.sin(), -rot.cos());

				let vel = dir * (missile.base_speed + speed * missile.speed_factor) * upg_factor;

				let missile_trajectory =
					MissileTrajectory(*self.pos.get(owner).unwrap(), missile.distance);

				let missile = self
					.entities
					.build_entity()
					.with(pos, &mut self.pos)
					.with(vel, &mut self.vel)
					.with(info.ty, &mut self.mob)
					.with(IsMissile, &mut self.is_missile)
					.with(PlayerRef(owner), &mut self.owner)
					.with(team, &mut self.team)
					.with(spawn_time, &mut self.spawn_time)
					.with(missile_trajectory, &mut self.missile_trajectory)
					.build();

				trace!(
					target: "missile-fire",
					"{:?} fired missile with id {:?}",
					owner, missile
				);

				missile
			}).collect::<Vec<_>>();

		self.channel.single_write(MissileFire {
			player: owner,
			missiles,
		});
	}
}

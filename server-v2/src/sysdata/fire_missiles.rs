use crate::component::{
    flag::{IsMissile, IsPlayer},
    time::MobSpawnTime,
    MissileOwner, Upgrades,
};
use crate::ecs::prelude::*;
use crate::event::MissileFire;
use crate::protocol::Vector2;
use crate::resource::{channel::OnMissileFire, Config, CurrentFrame};
use crate::sysdata::IsAlive;
use crate::{Mob, Position, Rotation, Team, Velocity};

pub struct FireMissileInfo {
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
    entities: Entities<'a>,

    current: ReadExpect<'a, CurrentFrame>,
    channel: Write<'a, OnMissileFire>,
    config: Read<'a, Config>,

    is_player: ReadStorage<'a, IsPlayer>,
    is_missile: WriteStorage<'a, IsMissile>,
    is_alive: IsAlive<'a>,
    upgrades: ReadStorage<'a, Upgrades>,

    team: WriteStorage<'a, Team>,
    pos: WriteStorage<'a, Position>,
    rot: ReadStorage<'a, Rotation>,
    vel: WriteStorage<'a, Velocity>,
    owner: WriteStorage<'a, MissileOwner>,
    mob: WriteStorage<'a, Mob>,
    spawn_time: WriteStorage<'a, MobSpawnTime>,
}

impl<'a> FireMissiles<'a> {
    pub fn fire_missiles<I>(&mut self, owner: Entity, missiles: I)
    where
        I: IntoIterator<Item = FireMissileInfo>,
    {
        self.is_player
            .get(owner)
            .expect("Non-player entity tried to fire a missile");

        if !self.is_alive.get(owner) {
            panic!("Player trying to firea missile is not alive");
        }

        let rot = *try_get!(owner, self.rot);
        let vel = *try_get!(owner, self.vel);
        let pos = *try_get!(owner, self.pos);
        let team = *try_get!(owner, self.team);
        let upg_factor =
            self.config.upgrades.missile.factor[try_get!(owner, self.upgrades).missile as usize];
        let speed = vel.length();
        let spawn_time = MobSpawnTime(self.current.0);

        let missiles = missiles
            .into_iter()
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

                let vel = Vector2::broadcast(
                    (missile.base_speed + speed * missile.speed_factor) * upg_factor,
                ) * dir;

                let builder = self.entities.build();
                builder
                    .with(&mut self.pos, pos)
                    .with(&mut self.vel, vel)
                    .with(&mut self.mob, info.ty)
                    .with(&mut self.is_missile, IsMissile)
                    .with(
                        &mut self.owner,
                        MissileOwner(self.entities.borrow(owner).unwrap()),
                    )
                    .with(&mut self.team, team)
                    .with(&mut self.spawn_time, spawn_time);
                let missile = builder.build();

                info!(
                    target: "missile-fire",
                    "{:?} fired missile with id {:?}",
                    owner, missile
                );

                missile
            })
            .collect();

        self.channel.single_write(MissileFire {
            player: self.entities.borrow(owner).unwrap(),
            missiles,
        })
    }
}

use crate::ecs::prelude::*;
use crate::event::MissileFire;
use crate::protocol::server::{PlayerFire, PlayerFireProjectile};
use crate::resource::Config;
use crate::sysdata::{Connections, ReadClock};
use crate::{Energy, EnergyRegen, Mob, Position, Velocity};

#[event_handler]
fn missile_notify<'a>(
    evt: &MissileFire,

    mob: &ReadStorage<'a, Mob>,
    pos: &ReadStorage<'a, Position>,
    vel: &ReadStorage<'a, Velocity>,
    energy: &ReadStorage<'a, Energy>,
    regen: &ReadStorage<'a, EnergyRegen>,

    clock: &ReadClock<'a>,
    config: &Read<'a, Config>,
    conns: &Connections<'a>,
) {
    let player = evt.player.entity();

    info!("Player {} fired missiles", player.id());

    let projectiles = evt
        .missiles
        .iter()
        .copied()
        .filter_map(|missile| {
            let ty = *(mob.get(missile)?);
            let vel = *(vel.get(missile)?);
            let info = config.mobs[ty].missile.as_ref()?;

            Some(PlayerFireProjectile {
                id: missile.into(),
                ty,
                pos: *(pos.get(missile)?),
                speed: vel,
                accel: vel.normalized() * info.accel,
                max_speed: info.max_speed,
            })
        })
        .collect();

    let pos = *try_get!(player, pos);
    conns.send_to_visible(
        pos,
        PlayerFire {
            clock: clock.ticks(),
            id: player.into(),
            energy: *try_get!(player, energy),
            energy_regen: *try_get!(player, regen),
            projectiles,
        },
    );
}

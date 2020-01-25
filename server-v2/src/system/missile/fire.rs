use crate::component::{time::LastShotTime, KeyState, PowerupExt, Powerups};
use crate::ecs::prelude::*;
use crate::resource::{Config, CurrentFrame};
use crate::sysdata::{FireMissileInfo, FireMissiles, IsAlive};
use crate::*;

/// When a player is capable of firing a missile, fire a missile.
///
/// Capable of firing a missile requires the following:
/// 1. The player is pressing the fire key.
/// 2. The player has enough energy to fire a missile.
/// 3. It has been long enough since firing the last missile.
#[system]
fn missile_fire<'a>(
    mut fire_missiles: FireMissiles<'a>,

    plane: ReadStorage<'a, Plane>,
    keystate: ReadStorage<'a, KeyState>,
    lastshot: ReadStorage<'a, LastShotTime>,
    powerups: ReadStorage<'a, Powerups>,
    mut energy: WriteStorage<'a, Energy>,

    config: Read<'a, Config>,
    current: ReadExpect<'a, CurrentFrame>,
    is_alive: IsAlive<'a>,
    entities: Entities<'a>,
) {
    let missiles = (
        &entities,
        &plane,
        &keystate,
        &mut energy,
        &lastshot,
        is_alive.mask(),
    )
        .join()
        .filter(|(_, _, keystate, ..)| keystate.fire)
        .filter_map(|(ent, &plane, _, energy, lastshot, ..)| {
            let info = &config.planes[plane];

            if current.0 - lastshot.0 > info.fire_delay {
                Some((ent, info, energy))
            } else {
                None
            }
        })
        .filter(|(_, info, energy)| **energy > info.fire_energy)
        .map(|(ent, info, energy)| {
            *energy -= info.fire_energy;

            let inferno = match powerups.get(ent) {
                Some(powerups) => powerups.inferno(),
                None => false,
            };

            let fire_info = if inferno {
                vec![
                    FireMissileInfo {
                        pos_offset: Position::new(
                            info.missile_inferno_offset_x,
                            info.missile_inferno_offset_y,
                        ),
                        rot_offset: -info.missile_inferno_angle,
                        ty: info.missile_type,
                    },
                    FireMissileInfo {
                        pos_offset: Position::new(Distance::default(), info.missile_offset),
                        rot_offset: Rotation::default(),
                        ty: info.missile_type,
                    },
                    FireMissileInfo {
                        pos_offset: Position::new(
                            -info.missile_inferno_offset_x,
                            info.missile_inferno_offset_y,
                        ),
                        rot_offset: info.missile_inferno_angle,
                        ty: info.missile_type,
                    },
                ]
            } else {
                vec![FireMissileInfo {
                    pos_offset: Position::new(Distance::default(), info.missile_offset),
                    rot_offset: Rotation::default(),
                    ty: info.missile_type,
                }]
            };

            (ent, fire_info)
        });

    for (ent, fire_info) in missiles {
        fire_missiles.fire_missiles(ent, fire_info);
    }
}

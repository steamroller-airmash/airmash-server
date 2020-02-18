use crate::ecs::prelude::*;
use crate::event::{collision::MissileTerrainCollision, MissileDespawn, MissileDespawnType};
use crate::protocol::{
    server::{MobDespawn, MobDespawnCoords},
    DespawnType,
};
use crate::resource::channel::OnMissileDespawn;
use crate::sysdata::Connections;
use crate::Mob;

/// When a missile collides with terrain, issue a
/// `MissileDespawn` event.
#[event_handler]
fn despawn_terrain_collision<'a>(
    evt: &MissileTerrainCollision,
    entities: &Entities<'a>,
    mob: &ReadStorage<'a, Mob>,
    channel: &mut Write<'a, OnMissileDespawn>,
) {
    let missile = match evt.missile.ent {
        Some(missile) => missile,
        None => {
            error!(
                "Missile-Terrain collision without missile entity: {:#?}",
                evt
            );
            return;
        }
    };

    channel.single_write(MissileDespawn {
        missile: match entities.borrow(missile) {
            Ok(borrowed) => borrowed,
            Err(e) => {
                error!("Missile-Terrain collision contained a dead entity: {}", e);
                return;
            }
        },
        ty: MissileDespawnType::HitTerrain,
        pos: evt.missile.pos,
        mob: *try_get!(missile, mob),
    });
}

#[event_handler]
fn missile_despawn<'a>(evt: &MissileDespawn, entities: &Entities<'a>, conns: &Connections<'a>) {
    use self::MissileDespawnType::*;

    if let Err(e) = entities.delete(evt.missile.entity()) {
        warn!("Got missile despawn event for dead missile: {}", e);
    }

    let despawn = match evt.ty {
        HitTerrain | HitPlayer => {
            conns.send_to_visible(
                evt.pos,
                MobDespawnCoords {
                    id: evt.missile.entity().into(),
                    ty: evt.mob,
                    pos: evt.pos,
                },
            );
            DespawnType::Collided
        }
        LifetimeEnded => DespawnType::LifetimeEnded,
    };

    conns.send_to_visible(
        evt.pos,
        MobDespawn {
            id: evt.missile.entity().into(),
            ty: despawn,
        },
    );
}

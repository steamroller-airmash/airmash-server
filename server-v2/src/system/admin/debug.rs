use crate::component::flag::{IsMissile, IsPlayer, IsPowerup};
use crate::component::*;
use crate::ecs::prelude::*;
use crate::protocol::{client::Command, server::CommandReply, CommandReplyType};
use crate::resource::{packet::ClientPacket, Config};
use crate::sysdata::Connections;

use std::borrow::Cow;

/// Admin command to dump some part of the fields of an entity.
///
/// Feel free to add new fields here as desired.
#[event_handler]
fn dump_entity<'a>(
    evt: &ClientPacket<Command<'static>>,

    entities: &Entities<'a>,
    config: &Read<'a, Config>,
    conns: &Connections<'a>,

    pos: &ReadStorage<'a, Position>,
    rot: &ReadStorage<'a, Rotation>,
    vel: &ReadStorage<'a, Velocity>,
    team: &ReadStorage<'a, Team>,
    plane: &ReadStorage<'a, Plane>,
    mob: &ReadStorage<'a, Mob>,
    health: &ReadStorage<'a, Health>,
    energy: &ReadStorage<'a, Energy>,

    is_missile: &ReadStorage<'a, IsMissile>,
    is_player: &ReadStorage<'a, IsPlayer>,
    is_powerup: &ReadStorage<'a, IsPowerup>,
) {
    if !config.admin_enabled {
        return;
    }

    if evt.packet.com != "debug-entity" {
        return;
    }

    let player = match conns.player(evt.connection) {
        Ok(Some(p)) => p,
        _ => return,
    };

    let target: u16 = match evt.packet.data.parse() {
        Ok(tgt) => tgt,
        Err(e) => {
            // TODO: Actually send an error message back to the player
            info!("Got invalid dump-entity command: {}", e);
            return;
        }
    };

    let target = match target {
        0 => player,
        _ => entities.forge(target as u32),
    };

    let data: Option<EntityData> = (|| {
        if !entities.is_alive(target) {
            return Some(EntityData::Dead);
        }

        if is_player.get(target).is_some() {
            return Some(EntityData::Player {
                pos: *(pos.get(target)?),
                rot: *(rot.get(target)?),
                vel: *(vel.get(target)?),
                team: *(team.get(target)?),
                plane: *(plane.get(target)?),
                energy: *(energy.get(target)?),
                health: *(health.get(target)?),
            });
        }

        if is_powerup.get(target).is_some() {
            return Some(EntityData::Powerup {
                pos: *(pos.get(target)?),
                ty: *(mob.get(target)?),
            });
        }

        if is_missile.get(target).is_some() {
            return Some(EntityData::Missile {
                pos: *(pos.get(target)?),
                vel: *(vel.get(target)?),
                team: *(team.get(target)?),
                ty: *(mob.get(target)?),
            });
        }

        Some(EntityData::Unknown {
            pos: pos.get(target).copied(),
        })
    })();

    let data = data.unwrap_or(EntityData::Error);

    conns.send_to(
        evt.connection,
        CommandReply {
            ty: CommandReplyType::ShowInConsole,
            text: Cow::Owned(
                serde_json::to_string_pretty(&data).expect("Failed to convert EntityData to JSON"),
            ),
        },
    );
}

#[derive(Serialize)]
#[serde(untagged)]
enum EntityData {
    Player {
        pos: Position,
        rot: Rotation,
        vel: Velocity,
        team: Team,
        plane: Plane,
        energy: Energy,
        health: Health,
    },
    Powerup {
        pos: Position,
        ty: Mob,
    },
    Missile {
        pos: Position,
        vel: Velocity,
        team: Team,
        ty: Mob,
    },
    Unknown {
        pos: Option<Position>,
    },
    Dead,
    Error,
}

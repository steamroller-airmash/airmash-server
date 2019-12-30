use crate::component::*;
use crate::ecs::{prelude::*, Join};
use crate::event::PlayerJoin;
use crate::protocol::{
    server::{Login, LoginPlayer},
    Upgrades as ProtocolUpgrades,
};
use crate::sysdata::{Connections, ReadClock};
use crate::util::{GameMode, GameModeWriter};

#[derive(SystemData)]
pub struct SendLoginData<'a> {
    entities: Entities<'a>,
    gamemode: GameModeWriter<'a, dyn GameMode>,
    clock: ReadClock<'a>,

    pos: ReadStorage<'a, Position>,
    rot: ReadStorage<'a, Rotation>,
    plane: ReadStorage<'a, Plane>,
    team: ReadStorage<'a, Team>,
    status: ReadStorage<'a, Status>,
    flag: ReadStorage<'a, Flag>,
    upgrades: ReadStorage<'a, Upgrades>,
    powerups: ReadStorage<'a, Powerups>,
    name: ReadStorage<'a, Name>,
    level: ReadStorage<'a, Level>,
    session: ReadStorage<'a, Session>,
}

#[event_handler]
fn send_login<'a>(evt: &PlayerJoin, data: &SendLoginData<'a>, conns: &Connections<'a>) {
    let player_data: Vec<_> = (
        &data.entities,
        &data.pos,
        &data.rot,
        &data.plane,
        &data.name,
        &data.flag,
        &data.upgrades,
        &data.level,
        &data.status,
        &data.team,
    )
        .join()
        .map(
            |(ent, pos, rot, plane, name, flag, upgrades, level, status, team)| {
                let powerups = data.powerups.get(ent);
                let upgrade_field = ProtocolUpgrades {
                    speed: upgrades.speed,
                    shield: powerups.shield(),
                    inferno: powerups.inferno(),
                };

                LoginPlayer {
                    id: ent.into(),
                    status: *status,
                    level: *level,
                    name: name.0.clone(),
                    ty: *plane,
                    team: *team,
                    pos: *pos,
                    rot: *rot,
                    flag: *flag,
                    upgrades: upgrade_field,
                }
            },
        )
        .collect();

    let gamemode = data.gamemode.get();
    let packet = Login {
        clock: data.clock.get(),
        id: evt.id.into(),
        room: gamemode.room(),
        success: true,
        token: try_get!(evt.id, data.session)
            .0
            .map(|x| x.to_string())
            .unwrap_or_else(|| "none".to_owned()),
        team: *try_get!(evt.id, data.team),
        ty: gamemode.gametype(),
        players: player_data,
    };

    conns.send_to_player(evt.id, packet);
}

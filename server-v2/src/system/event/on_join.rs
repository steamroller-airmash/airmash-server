use crate::component::counter::{Earnings, TotalDeaths, TotalKills};
use crate::component::*;
use crate::ecs::{prelude::*, Builder, Join};
use crate::event::{PlayerJoin, PlayerPowerup};
use crate::protocol::{
    server::{Login, LoginPlayer, PlayerLevel, PlayerNew, ScoreUpdate},
    PlayerLevelType, Upgrades as ProtocolUpgrades,
};
use crate::resource::{channel::OnPlayerPowerup, Config};
use crate::sysdata::{Connections, ReadClock};
use crate::util::{GameMode, GameModeWriter};

use std::borrow::Cow;

pub fn register(builder: &mut Builder) {
    builder
        .with::<send_login>()
        .with::<send_level>()
        .with::<send_player_new>()
        .with::<send_player_powerup>()
        .with::<send_score_update>();
}

#[derive(SystemData)]
struct SendLoginData<'a> {
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
                    name: Cow::Borrowed(&*name.0),
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
        clock: data.clock.ticks(),
        id: evt.id.into(),
        room: Cow::Owned(gamemode.room()),
        success: true,
        token: try_get!(evt.id, data.session)
            .0
            .map(|x| Cow::Owned(x.to_string()))
            .unwrap_or_else(|| Cow::Borrowed("none")),
        team: *try_get!(evt.id, data.team),
        ty: gamemode.gametype(),
        players: player_data,
    };

    info!("Send login to {:?}", evt.id);

    conns.send_to_player(evt.id, packet);
}

#[event_handler(deps = send_player_new)]
fn send_level<'a>(evt: &PlayerJoin, conns: &Connections<'a>, level: &ReadStorage<'a, Level>) {
    let packet = PlayerLevel {
        id: evt.id.into(),
        ty: PlayerLevelType::Login,
        level: *try_get!(evt.id, level),
    };

    conns.send_to_others(evt.id, packet);
}

#[derive(SystemData)]
struct SendPlayerNewData<'a> {
    conns: Connections<'a>,

    pos: ReadStorage<'a, Position>,
    rot: ReadStorage<'a, Rotation>,
    plane: ReadStorage<'a, Plane>,
    team: ReadStorage<'a, Team>,
    status: ReadStorage<'a, Status>,
    flag: ReadStorage<'a, Flag>,
    upgrades: ReadStorage<'a, Upgrades>,
    powerups: ReadStorage<'a, Powerups>,
    name: ReadStorage<'a, Name>,
}

#[event_handler]
fn send_player_new<'a>(evt: &PlayerJoin, data: &SendPlayerNewData<'a>) {
    let powerups = data.powerups.get(evt.id);

    let upgrades = ProtocolUpgrades {
        speed: try_get!(evt.id, data.upgrades).speed,
        inferno: powerups.inferno(),
        shield: powerups.shield(),
    };

    let player_new = PlayerNew {
        id: evt.id.into(),
        status: *try_get!(evt.id, data.status),
        name: Cow::Borrowed(&try_get!(evt.id, data.name).0),
        ty: *try_get!(evt.id, data.plane),
        team: *try_get!(evt.id, data.team),
        pos: *try_get!(evt.id, data.pos),
        rot: *try_get!(evt.id, data.rot),
        flag: *try_get!(evt.id, data.flag),
        upgrades,
    };

    data.conns.send_to_others(evt.id, player_new);
}

#[event_handler(deps = send_player_new)]
fn send_player_powerup<'a>(
    evt: &PlayerJoin,
    config: &Read<'a, Config>,
    channel: &mut Write<'a, OnPlayerPowerup>,
) {
    channel.single_write(PlayerPowerup {
        player: evt.id,
        duration: config.spawn_shield_duration,
        ty: Powerup::Shield,
    });
}

#[derive(SystemData)]
struct SendScoreUpdateData<'a> {
    conns: Connections<'a>,

    score: ReadStorage<'a, Score>,
    earnings: ReadStorage<'a, Earnings>,
    upgrades: ReadStorage<'a, Upgrades>,
    total_kills: ReadStorage<'a, TotalKills>,
    total_deaths: ReadStorage<'a, TotalDeaths>,
}

#[event_handler(deps = (send_player_new, send_login))]
fn send_score_update<'a>(evt: &PlayerJoin, data: &SendScoreUpdateData<'a>) {
    let score = try_get!(evt.id, data.score);
    let earnings = try_get!(evt.id, data.earnings);
    let upgrades = try_get!(evt.id, data.upgrades);
    let total_kills = try_get!(evt.id, data.total_kills);
    let total_deaths = try_get!(evt.id, data.total_deaths);

    let packet = ScoreUpdate {
        id: evt.id.into(),
        score: *score,
        earnings: earnings.0,
        upgrades: upgrades.unused,
        total_kills: total_kills.0,
        total_deaths: total_deaths.0,
    };

    data.conns.send_to_all(packet);
}

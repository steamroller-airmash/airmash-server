use crate::component::*;
use crate::component::{
    flag::{ForcePlayerUpdate, IsPlayer},
    time::LastUpdate,
};
use crate::ecs::prelude::*;
use crate::protocol::{server::PlayerUpdate, Upgrades as ServerUpgrades};
use crate::resource::{Config, CurrentFrame, LastFrame};
use crate::sysdata::{Connections, IsAlive, ReadClock};

use std::f32::consts;
use std::time::Duration;

const PI: Rotation = Rotation::new(consts::PI);
// PIX2 is less clear
#[allow(non_upper_case_globals)]
const PIx2: Rotation = Rotation::new(2.0 * consts::PI);
const FRAC_PI_2: Rotation = Rotation::new(consts::FRAC_PI_2);

#[derive(SystemData)]
struct PositionUpdateData<'a> {
    pos: WriteStorage<'a, Position>,
    rot: WriteStorage<'a, Rotation>,
    vel: WriteStorage<'a, Velocity>,
    team: ReadStorage<'a, Team>,
    keystate: ReadStorage<'a, KeyState>,
    upgrades: ReadStorage<'a, Upgrades>,
    powerups: ReadStorage<'a, Powerups>,
    planes: ReadStorage<'a, Plane>,
    force_update: WriteStorage<'a, ForcePlayerUpdate>,
    is_player: ReadStorage<'a, IsPlayer>,

    lastframe: ReadExpect<'a, LastFrame>,
    thisframe: ReadExpect<'a, CurrentFrame>,
    entities: Entities<'a>,
    is_alive: IsAlive<'a>,
    conns: Connections<'a>,
    clock: ReadClock<'a>,
}

#[system]
fn update_positions<'a>(
    mut data: PositionUpdateData<'a>,
    config: Read<'a, Config>,
    mut lastupdate: WriteStorage<'a, LastUpdate>,
) {
    step_players(&mut data, &config);
    send_updates(&mut data, &mut lastupdate);
    send_outdated(&mut data, &mut lastupdate);

    data.force_update.clear();
}

fn step_players(data: &mut PositionUpdateData, config: &Config) {
    let delta: Time = Time::from(data.thisframe.0 - data.lastframe.0);

    let PositionUpdateData {
        entities,
        pos,
        rot,
        vel,
        keystate,
        upgrades,
        powerups,
        planes,
        is_alive,
        is_player,
        ..
    } = data;

    let iter = (
        &*entities,
        pos,
        rot,
        vel,
        &*keystate,
        &*upgrades,
        &*planes,
        is_alive.mask() & is_player.mask(),
    )
        .join()
        .map(|(ent, pos, rot, vel, keystate, upgrades, plane, ..)| {
            let powerups = powerups.get(ent);
            (pos, rot, vel, keystate, upgrades, powerups, plane)
        });

    for (pos, rot, vel, keystate, upgrades, powerups, plane) in iter {
        let mut movement_angle = None;
        let info = &config.planes[*plane];
        let boost_factor = if keystate.boost(&plane) {
            info.boost_factor
        } else {
            1.0
        };

        if keystate.strafe(plane) {
            if keystate.left {
                movement_angle = Some(*rot - FRAC_PI_2);
            }
            if keystate.right {
                movement_angle = Some(*rot + FRAC_PI_2);
            }
        } else {
            if keystate.left {
                *rot += -delta * info.turn_factor;
            }
            if keystate.right {
                *rot += delta * info.turn_factor;
            }
        }

        if keystate.up {
            if let Some(angle) = movement_angle {
                if keystate.right {
                    movement_angle = Some(angle + PI * (-0.25));
                } else if keystate.left {
                    movement_angle = Some(angle + PI * (0.25));
                }
            } else {
                movement_angle = Some(*rot);
            }
        } else if keystate.down {
            if let Some(angle) = movement_angle {
                if keystate.right {
                    movement_angle = Some(angle + PI * (0.25));
                } else if keystate.left {
                    movement_angle = Some(angle + PI * (-0.25));
                }
            } else {
                movement_angle = Some(*rot + PI);
            }
        }

        if let Some(angle) = movement_angle {
            let mult = info.accel_factor * delta * boost_factor;
            *vel += Velocity::new(mult * angle.sin(), mult * -angle.cos());
        }

        let oldspeed: Velocity = *vel;
        let speed_len: Speed = vel.length();
        let mut max_speed: Speed = info.max_speed * boost_factor;
        let min_speed: Speed = info.min_speed;

        // Need to fill out config more
        if upgrades.speed != 0 {
            max_speed *= config.upgrades.speed.factor[upgrades.speed as usize]
        }

        if powerups.inferno() {
            max_speed *= info.inferno_factor;
        }

        if keystate.flagspeed {
            max_speed = info.flag_speed;
        }

        if speed_len > max_speed {
            *vel *= max_speed / speed_len;
        } else {
            if vel.x.abs() > min_speed || vel.y.abs() > min_speed {
                *vel *= 1.0 - (info.brake_factor * delta).inner();
            } else {
                *vel = Velocity::default()
            }
        }

        *pos += oldspeed * delta + (*vel - oldspeed) * delta * 0.5;
        *rot = (*rot % PIx2 + PIx2) % PIx2;

        let bound = Position::new(Distance::new(16352.0), Distance::new(8160.0));

        if pos.x.abs() > bound.x {
            pos.x = bound.x * pos.x.signum();
        }
        if pos.y.abs() > bound.y {
            pos.y = bound.y * pos.y.signum()
        }
    }
}

fn send_updates(data: &mut PositionUpdateData, lastupdate: &mut WriteStorage<LastUpdate>) {
    let clock = data.clock.get();
    let thisframe = data.thisframe.0;

    let iter = (
        &data.entities,
        &data.pos,
        &data.rot,
        &data.vel,
        &data.planes,
        &data.keystate,
        &data.upgrades,
        lastupdate,
        // Update if forced to do so
        data.force_update.mask() & data.is_alive.mask(),
    )
        .join()
        .map(|x| x)
        .map(
            |(ent, pos, rot, vel, plane, keystate, upgrades, lastupdate, ..)| {
                let powerups = data.powerups.get(ent);
                (
                    ent, pos, rot, vel, plane, keystate, upgrades, powerups, lastupdate,
                )
            },
        );

    for (ent, pos, rot, vel, plane, keystate, upgrades, powerups, lastupdate) in iter {
        *lastupdate = LastUpdate(thisframe);

        let state = keystate.to_server(&plane);

        let ups = ServerUpgrades {
            speed: upgrades.speed,
            shield: powerups.shield(),
            inferno: powerups.inferno(),
        };

        let packet = PlayerUpdate {
            clock,
            id: ent.into(),
            keystate: state,
            pos: *pos,
            rot: *rot,
            speed: *vel,
            upgrades: ups,
        };

        trace!(target: "airmash:position_update", "Update: {:?}", packet);

        if !keystate.stealthed {
            data.conns.send_to_visible(packet.pos, packet);
        } else {
            let team = *try_get!(ent, data.team);
            data.conns.send_to_team(team, packet);
        }
    }
}

fn send_outdated(data: &mut PositionUpdateData, lastupdate: &mut WriteStorage<LastUpdate>) {
    let clock = data.clock.get();

    let iter = (
        lastupdate,
        &data.pos,
        &data.rot,
        &data.vel,
        &data.planes,
        &data.keystate,
        &data.upgrades,
        &data.entities,
        data.is_alive.mask(),
    )
        .join()
        .filter(|(lastupdate, ..)| lastupdate.0.elapsed() > Duration::from_secs(1))
        .map(
            |(lastupdate, pos, rot, vel, plane, keystate, upgrades, ent, ..)| {
                let powerups = data.powerups.get(ent);
                (
                    pos, rot, vel, plane, keystate, upgrades, powerups, ent, lastupdate,
                )
            },
        );

    for (pos, rot, vel, plane, keystate, upgrades, powerups, ent, lastupdate) in iter {
        *lastupdate = LastUpdate(data.thisframe.0);

        let state = keystate.to_server(&plane);

        let ups = ServerUpgrades {
            speed: upgrades.speed,
            shield: powerups.shield(),
            inferno: powerups.inferno(),
        };

        let packet = PlayerUpdate {
            clock,
            id: ent.into(),
            keystate: state,
            pos: *pos,
            rot: *rot,
            speed: *vel,
            upgrades: ups,
        };

        trace!(target: "airmash:position_update", "Update: {:?}", packet);

        if !keystate.stealthed {
            data.conns.send_to_visible(*pos, packet);
        } else {
            let team = *try_get!(ent, data.team);
            data.conns.send_to_team(team, packet);
        }
    }
}

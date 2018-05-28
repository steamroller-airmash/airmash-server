use specs::*;
use types::*;

use std::f32::consts;
use std::marker::PhantomData;

use airmash_protocol::server::{PlayerUpdate, ServerPacket};
use airmash_protocol::{to_bytes, PlaneType, ServerKeyState, Upgrades as ServerUpgrades};
use websocket::OwnedMessage;

const PI: Rotation = Rotation {
    value_unsafe: consts::PI,
    _marker: PhantomData,
};
// PIX2 is less clear
#[allow(non_upper_case_globals)]
const PIx2: Rotation = Rotation {
    value_unsafe: 2.0 * consts::PI,
    _marker: PhantomData,
};
const FRAC_PI_2: Rotation = Rotation {
    value_unsafe: consts::FRAC_PI_2,
    _marker: PhantomData,
};

pub struct PositionUpdate {
    dirty: BitSet,
    modify_reader: Option<ReaderId<ModifiedFlag>>,
}

impl PositionUpdate {
    pub fn new() -> Self {
        Self {
            dirty: BitSet::default(),
            modify_reader: None,
        }
    }

    fn boost(plane: &Plane, keystate: &KeyState) -> bool {
        plane.0 == PlaneType::Predator && keystate.special
    }
    fn strafe(plane: &Plane, keystate: &KeyState) -> bool {
        plane.0 == PlaneType::Mohawk && keystate.special
    }

    fn step_players<'a>(data: &mut PositionUpdateData<'a>, config: &Read<'a, Config>) {
        let delta = Time::from(data.thisframe.0 - data.lastframe.0) * 60.0;

        (
            &mut data.pos,
            &mut data.rot,
            &mut data.speed,
            &data.keystate,
            &data.upgrades,
            &data.powerups,
            &data.planes,
        ).join()
            .for_each(|(pos, rot, speed, keystate, upgrades, powerups, plane)| {
                let mut movement_angle = None;
                let info = &config.planes[*plane];
                let boost_factor = if Self::boost(&plane, keystate) {
                    info.boost_factor
                } else {
                    1.0
                };

                if Self::strafe(plane, keystate) {
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
                            movement_angle = Some(angle + PI * (0.25));
                        } else if keystate.left {
                            movement_angle = Some(angle + PI * (-0.25));
                        }
                    } else {
                        movement_angle = Some(*rot);
                    }
                } else if keystate.down {
                    if let Some(angle) = movement_angle {
                        if keystate.right {
                            movement_angle = Some(angle + PI * (-0.25));
                        } else if keystate.left {
                            movement_angle = Some(angle + PI * (0.25));
                        }
                    } else {
                        movement_angle = Some(*rot + PI);
                    }
                }

                if let Some(angle) = movement_angle {
                    let mult = info.accel_factor * delta * boost_factor;
                    *speed += Vector2::new(mult * angle.sin(), mult * -angle.cos());
                }

                let oldspeed = *speed;
                let speed_len = speed.length();
                let mut max_speed = info.max_speed * boost_factor;
                let min_speed = info.min_speed;

                // Need to fill out config more
                if upgrades.speed != 0 {
                    unimplemented!();
                }

                if powerups.inferno {
                    max_speed *= info.inferno_factor;
                }

                if speed_len > max_speed {
                    *speed *= max_speed / speed_len;
                } else {
                    if speed.x.abs() > min_speed || speed.y.abs() > min_speed {
                        let val = 1.0 - (info.brake_factor * delta).inner();
                        *speed *= val;
                    } else {
                        *speed = Speed::default()
                    }
                }

                *pos += oldspeed * delta + (*speed - oldspeed) * delta * 0.5;
                *rot = (*rot % PIx2 + PIx2) % PIx2;

                let bound = Position::new(Distance::new(16352.0), Distance::new(8160.0));

                if pos.x.abs() > bound.x {
                    pos.x = pos.x.signum() * bound.x
                }
                if pos.y.abs() > bound.y {
                    pos.y = pos.y.signum() * bound.y
                }
            });
    }

    fn send_updates<'a>(&self, data: &mut PositionUpdateData<'a>) {
        (
            &data.pos,
            &data.rot,
            &data.speed,
            &data.planes,
            &data.keystate,
            &data.upgrades,
            &data.powerups,
            &*data.entities,
            &self.dirty,
        ).join()
            .for_each(
                |(pos, rot, speed, plane, keystate, upgrades, powerups, ent, _)| {
                    type Key = ServerKeyState;

                    let mut state = ServerKeyState(0);
                    state.set(Key::UP, keystate.up);
                    state.set(Key::DOWN, keystate.down);
                    state.set(Key::LEFT, keystate.left);
                    state.set(Key::RIGHT, keystate.right);
                    state.set(Key::BOOST, Self::boost(plane, keystate));
                    state.set(Key::STRAFE, Self::strafe(plane, keystate));
                    state.set(Key::STEALTH, keystate.stealthed);
                    state.set(Key::FLAGSPEED, keystate.flagspeed);

                    let mut ups = ServerUpgrades(0);
                    ups.set_speed(upgrades.speed);
                    ups.set(ServerUpgrades::INFERNO, powerups.inferno);
                    ups.set(ServerUpgrades::SHIELD, powerups.shield);

                    let packet = PlayerUpdate {
                        clock: (data.thisframe.0 - data.starttime.0).to_clock(),
                        id: ent.id() as u16,
                        keystate: state,
                        pos_x: pos.x.inner(),
                        pos_y: pos.y.inner(),
                        rot: rot.inner(),
                        speed_x: speed.x.inner(),
                        speed_y: speed.y.inner(),
                        upgrades: ups,
                    };

                    data.conns.send_to_all(OwnedMessage::Binary(
                        to_bytes(&ServerPacket::PlayerUpdate(packet)).unwrap(),
                    ))
                },
            )
    }
}

#[derive(SystemData)]
pub struct PositionUpdateData<'a> {
    pos: WriteStorage<'a, Position>,
    rot: WriteStorage<'a, Rotation>,
    speed: WriteStorage<'a, Speed>,
    keystate: ReadStorage<'a, KeyState>,
    upgrades: ReadStorage<'a, Upgrades>,
    powerups: ReadStorage<'a, Powerups>,
    planes: ReadStorage<'a, Plane>,
    lastframe: Read<'a, LastFrame>,
    thisframe: Read<'a, ThisFrame>,
    starttime: Read<'a, StartTime>,
    entities: Entities<'a>,
    conns: Read<'a, Connections>,
}

impl<'a> System<'a> for PositionUpdate {
    type SystemData = (PositionUpdateData<'a>, Read<'a, Config>);

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        let mut storage: WriteStorage<KeyState> = SystemData::fetch(&res);
        self.modify_reader = Some(storage.track_modified());
    }

    fn run(&mut self, (mut data, config): Self::SystemData) {
        self.dirty.clear();
        data.keystate
            .populate_modified(&mut self.modify_reader.as_mut().unwrap(), &mut self.dirty);

        Self::step_players(&mut data, &config);
        self.send_updates(&mut data);
    }
}

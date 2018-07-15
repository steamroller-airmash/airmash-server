use specs::*;
use types::systemdata::*;
use types::*;

use component::time::*;

use std::f32::consts;
use std::marker::PhantomData;
use std::time::Duration;

use airmash_protocol::server::{PlayerUpdate, ServerPacket};
use airmash_protocol::{to_bytes, Upgrades as ServerUpgrades};
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

#[derive(SystemData)]
pub struct PositionUpdateData<'a> {
	pos: WriteStorage<'a, Position>,
	rot: WriteStorage<'a, Rotation>,
	vel: WriteStorage<'a, Velocity>,
	keystate: ReadStorage<'a, KeyState>,
	upgrades: ReadStorage<'a, Upgrades>,
	powerups: ReadStorage<'a, Powerups>,
	planes: ReadStorage<'a, Plane>,
	lastframe: Read<'a, LastFrame>,
	thisframe: Read<'a, ThisFrame>,
	entities: Entities<'a>,
	conns: Read<'a, Connections>,
	is_alive: IsAlive<'a>,
	clock: ReadClock<'a>,
}

impl PositionUpdate {
	pub fn new() -> Self {
		Self {
			dirty: BitSet::default(),
			modify_reader: None,
		}
	}

	fn step_players<'a>(data: &mut PositionUpdateData<'a>, config: &Read<'a, Config>) {
		let delta = Time::from(data.thisframe.0 - data.lastframe.0);

		let is_alive = &data.is_alive;

		(
			&mut data.pos,
			&mut data.rot,
			&mut data.vel,
			&data.keystate,
			&data.upgrades,
			&data.powerups,
			&data.planes,
			&*data.entities,
		).join()
			.filter(|(_, _, _, _, _, _, _, ent)| is_alive.get(*ent))
			.for_each(|(pos, rot, vel, keystate, upgrades, powerups, plane, _)| {
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
					*vel += Vector2::new(mult * angle.sin(), mult * -angle.cos());
				}

				let oldspeed = *vel;
				let speed_len = vel.length();
				let mut max_speed = info.max_speed * boost_factor;
				let min_speed = info.min_speed;

				// Need to fill out config more
				if upgrades.speed != 0 {
					unimplemented!();
				}

				if powerups.inferno {
					max_speed *= info.inferno_factor;
				}

				if keystate.flagspeed {
					max_speed = info.flag_speed;
				}

				if speed_len > max_speed {
					*vel *= max_speed / speed_len;
				} else {
					if vel.x.abs() > min_speed || vel.y.abs() > min_speed {
						let val = 1.0 - (info.brake_factor * delta).inner();
						*vel *= val;
					} else {
						*vel = Velocity::default()
					}
				}

				*pos += oldspeed * delta + (*vel - oldspeed) * delta * 0.5;
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

	fn send_updates<'a>(
		&self,
		data: &mut PositionUpdateData<'a>,
		lastupdate: &mut WriteStorage<'a, LastUpdate>,
	) {
		let clock = data.clock.get();
		let thisframe = data.clock.frame.0;

		(
			&data.pos,
			&data.rot,
			&data.vel,
			&data.planes,
			&data.keystate,
			&data.upgrades,
			&data.powerups,
			&*data.entities,
			&self.dirty,
			lastupdate,
		).join()
			.filter(|(_, _, _, _, _, _, _, ent, _, _)| data.is_alive.get(*ent))
			.for_each(
				|(pos, rot, vel, plane, keystate, upgrades, powerups, ent, _, lastupdate)| {
					*lastupdate = LastUpdate(thisframe);

					let state = keystate.to_server(&plane);

					let ups = ServerUpgrades {
						speed: upgrades.speed,
						shield: powerups.shield,
						inferno: powerups.inferno,
					};

					let packet = PlayerUpdate {
						clock,
						id: ent,
						keystate: state,
						pos: *pos,
						rot: *rot,
						speed: *vel,
						upgrades: ups,
					};

					trace!(target: "server", "Update: {:?}", packet);

					let message = OwnedMessage::Binary(
						to_bytes(&ServerPacket::PlayerUpdate(packet)).unwrap(),
					);

					if !keystate.stealthed {
						data.conns.send_to_all(message);
					} else {
						data.conns.send_to_team(ent, message);
					}
				},
			)
	}

	fn send_outdated<'a>(
		&self,
		data: &mut PositionUpdateData<'a>,
		lastupdate: &mut WriteStorage<'a, LastUpdate>,
	) {
		let clock = data.clock.get();

		(
			&data.pos,
			&data.rot,
			&data.vel,
			&data.planes,
			&data.keystate,
			&data.upgrades,
			&data.powerups,
			&*data.entities,
			lastupdate,
		).join()
			.filter(|(_, _, _, _, _, _, _, _, lastupdate)| {
				lastupdate.0.elapsed() > Duration::from_secs(1)
			})
			.filter(|(_, _, _, _, _, _, _, ent, _)| data.is_alive.get(*ent))
			.for_each(
				|(pos, rot, vel, plane, keystate, upgrades, powerups, ent, lastupdate)| {
					*lastupdate = LastUpdate(data.thisframe.0);

					let state = keystate.to_server(&plane);

					let ups = ServerUpgrades {
						speed: upgrades.speed,
						shield: powerups.shield,
						inferno: powerups.inferno,
					};

					let packet = PlayerUpdate {
						clock,
						id: ent,
						keystate: state,
						pos: *pos,
						rot: *rot,
						speed: *vel,
						upgrades: ups,
					};

					trace!(target: "server", "Update: {:?}", packet);

					let message = OwnedMessage::Binary(
						to_bytes(&ServerPacket::PlayerUpdate(packet)).unwrap(),
					);

					if !keystate.stealthed {
						data.conns.send_to_all(message);
					} else {
						data.conns.send_to_team(ent, message);
					}
				},
			)
	}
}

impl<'a> System<'a> for PositionUpdate {
	type SystemData = (
		PositionUpdateData<'a>,
		Read<'a, Config>,
		WriteStorage<'a, LastUpdate>,
	);

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		let mut storage: WriteStorage<KeyState> = SystemData::fetch(&res);
		self.modify_reader = Some(storage.track_modified());
	}

	fn run(&mut self, (mut data, config, mut lastupdate): Self::SystemData) {
		self.dirty.clear();
		data.keystate
			.populate_modified(&mut self.modify_reader.as_mut().unwrap(), &mut self.dirty);

		Self::step_players(&mut data, &config);
		self.send_updates(&mut data, &mut lastupdate);
		self.send_outdated(&mut data, &mut lastupdate);
	}
}

use dispatch::SystemInfo;
use handlers::KeyHandler;

impl SystemInfo for PositionUpdate {
	type Dependencies = KeyHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

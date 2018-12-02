use specs::*;
use types::*;

use component::event::*;
use component::time::{StartTime, ThisFrame};

use utils::{EventHandler, EventHandlerTypeProvider};

use airmash_protocol::server::EventBounce;

#[derive(Default)]
pub struct BounceSystem;

#[derive(SystemData)]
pub struct BounceSystemData<'a> {
	vel: WriteStorage<'a, Velocity>,
	pos: ReadStorage<'a, Position>,
	rot: ReadStorage<'a, Rotation>,
	plane: ReadStorage<'a, Plane>,
	keystate: ReadStorage<'a, KeyState>,
	conns: Read<'a, Connections>,
	thisframe: Read<'a, ThisFrame>,
	starttime: Read<'a, StartTime>,
}

impl EventHandlerTypeProvider for BounceSystem {
	type Event = PlayerTerrainCollision;
}

impl<'a> EventHandler<'a> for BounceSystem {
	type SystemData = BounceSystemData<'a>;

	fn on_event(&mut self, evt: &PlayerTerrainCollision, data: &mut Self::SystemData) {
		let evt = evt.0;

		if evt.0.layer != 0 && evt.1.layer != 0 {
			return;
		}

		assert!(evt.1.layer != evt.0.layer);

		let rel;
		let maxspd;
		let ent;
		if evt.0.layer == 0 {
			ent = evt.1.ent;
			rel = (evt.1.pos - evt.0.pos).normalized();
			maxspd = *try_get!(evt.1.ent, data.vel);
		} else {
			ent = evt.0.ent;
			rel = (evt.0.pos - evt.1.pos).normalized();
			maxspd = *try_get!(evt.0.ent, data.vel);
		};

		let vel = rel * Speed::max(maxspd.length(), Speed::new(1.0));

		*try_get!(ent, mut data.vel) = vel;

		let pos = try_get!(ent, data.pos);
		let rot = try_get!(ent, data.rot);
		let keystate = try_get!(ent, data.keystate);
		let plane = try_get!(ent, data.plane);
		let state = keystate.to_server(&plane);

		let packet = EventBounce {
			clock: (data.thisframe.0 - data.starttime.0).to_clock(),
			id: ent.into(),
			pos: *pos,
			rot: *rot,
			speed: vel,
			keystate: state,
		};

		if keystate.stealthed {
			// Stealthed prowlers should not have position
			// updates sent to all visible players.
			// This should really be something like
			// send_to_team_visible
			data.conns.send_to_team(ent, packet);
		} else {
			data.conns.send_to_visible(*pos, packet);
		}
	}
}

use super::PlaneCollisionSystem;
use dispatch::SystemInfo;

impl SystemInfo for BounceSystem {
	type Dependencies = PlaneCollisionSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

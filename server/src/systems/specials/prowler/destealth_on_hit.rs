
use specs::*;
use types::*;

use SystemInfo;
use component::event::*;
use component::channel::*;
use systems::collision::PlayerMissileCollisionSystem;

pub struct DestealthOnHit {
	reader: Option<OnPlayerMissileCollisionReader>,
}

#[derive(SystemData)]
pub struct DestealthOnHitData<'a> {
	channel: Read<'a, OnPlayerMissileCollision>,

	keystate: WriteStorage<'a, KeyState>,
	plane: ReadStorage<'a, Plane>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl<'a> System<'a> for DestealthOnHit {
	type SystemData = DestealthOnHitData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerMissileCollision>()
				.register_reader()
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for PlayerMissileCollision(evt) in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = data.is_player.get(evt.0.ent)
				.map(|_| evt.0.ent)
				.unwrap_or(evt.1.ent);

			if *data.plane.get(player).unwrap() != Plane::Prowler {
				continue;
			}

			match data.keystate.get_mut(player) {
				Some(keystate) => keystate.stealthed = false,
				_ => ()
			}
		}
	}
}

impl SystemInfo for DestealthOnHit {
	type Dependencies = PlayerMissileCollisionSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self{ reader: None }
	}
}

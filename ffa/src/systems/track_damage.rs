
use specs::*;

use components::TotalDamage;

use airmash_server::*;
use airmash_server::component::channel::*;

#[derive(Default)]
pub struct TrackDamage {
	reader: Option<OnPlayerHitReader>,
}

#[derive(SystemData)]
pub struct TrackDamageData<'a> {
	entities: Entities<'a>,
	config: Read<'a, Config>,
	channel: Read<'a, OnPlayerHit>,

	mob: ReadStorage<'a, Mob>,

	damage: WriteStorage<'a, TotalDamage>,
}

impl<'a> System<'a> for TrackDamage {
	type SystemData = TrackDamageData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerHit>().register_reader()
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if !data.entities.is_alive(evt.player) { continue; }

			let mob = *data.mob.get(evt.missile).unwrap();
			let ref info = data.config.mobs[mob].missile.unwrap();

			data.damage.get_mut(evt.player).unwrap().0 += info.damage;
		}
	}
}

impl SystemInfo for TrackDamage {
	type Dependencies = super::AddDamage;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

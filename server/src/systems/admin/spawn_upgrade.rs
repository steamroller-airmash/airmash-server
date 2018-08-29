use specs::*;

use SystemInfo;

use types::*;
use utils::maybe_init::MaybeInit;

use component::channel::*;
use component::event::*;

use utils::event_handler::EventHandler;

#[derive(Default)]
pub struct SpawnUpgrade {
	reader: MaybeInit<OnCommandReader>,
}

#[derive(SystemData)]
pub struct SpawnUpgradeData<'a> {
	channel: Read<'a, OnCommand>,
}



impl<'a> System<'a> for SpawnUpgrade {
	type SystemData = SpawnUpgradeData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = MaybeInit::new(res.fetch_mut::<OnCommand>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {}
}

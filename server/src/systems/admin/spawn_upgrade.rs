use specs::*;

use SystemInfo;

use types::*;
use utils::maybe_init::MaybeInit;

use component::channel::*;
use component::event::*;

use utils::event_handler::*;

#[derive(Default)]
pub struct SpawnUpgrade {
	reader: MaybeInit<OnCommandReader>,
}

#[derive(SystemData)]
pub struct SpawnUpgradeData<'a> {
	channel: Write<'a, OnUpgradeSpawn>,
	config: Read<'a, Config>,

	entities: Entities<'a>,
	pos: WriteStorage<'a, Position>,
}

impl EventHandlerTypeProvider for SpawnUpgrade {
	type Event = CommandEvent;
}

fn unwrap_all<T, E>(iter: impl Iterator<Item = Result<T, E>>) -> Result<Vec<T>, E> {
	let mut vals = vec![];

	for item in iter {
		vals.push(item?);
	}

	Ok(vals)
}

impl<'a> EventHandler<'a> for SpawnUpgrade {
	type SystemData = SpawnUpgradeData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = MaybeInit::new(res.fetch_mut::<OnCommand>().register_reader());
	}

	fn on_event(&mut self, evt: &Self::Event, data: &mut Self::SystemData) {
		let (_, cmd) = evt;

		// If admin commands aren't enabled then
		// there is nothing to do here.
		if !data.config.admin_enabled {
			return;
		}

		if cmd.com != "spawn-upgrade" {
			return;
		}

		let iter = cmd.data.split(' ').take(2).map(|x| x.parse());

		let subvals: Vec<f32> = match unwrap_all(iter) {
			Ok(v) => v,
			Err(_) => {
				warn!("Admin command 'spawn-upgrade' submitted with an invalid argument!");
				return;
			}
		};

		// A malformed command was submitted (not enough coordinates)
		// Print a warning to console then ignore it, there isn't a
		// a way to send it back to the client at this point in time.
		if subvals.len() < 2 {
			warn!("Admin command 'spawn-upgrade' submitted with too few arguments!");
			return;
		}

		let pos = Position::new(subvals[0], subvals[1]);
		let upgrade = data.entities.create();

		data.pos.insert(upgrade, pos).unwrap();

		data.channel
			.single_write(UpgradeSpawnEvent { upgrade, pos });
	}
}

impl SystemInfo for SpawnUpgrade {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

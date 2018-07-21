use specs::*;

use super::*;
use component::*;

use server::component::channel::*;
use server::component::flag::*;
use server::component::time::*;
use server::systems::handlers::packet::CommandHandler;
use server::*;

pub struct DropOnDeath {
	pub reader: Option<OnPlayerKilledReader>,
}

#[derive(SystemData)]
pub struct DropOnDeathData<'a> {
	pub channel: Write<'a, OnFlag>,
	pub commands: Read<'a, OnPlayerKilled>,
	pub conns: Read<'a, Connections>,
	pub entities: Entities<'a>,
	pub thisframe: Read<'a, ThisFrame>,

	pub team: ReadStorage<'a, Team>,
	pub pos: ReadStorage<'a, Position>,
	pub lastdrop: WriteStorage<'a, LastDrop>,
	pub carrier: WriteStorage<'a, FlagCarrier>,

	pub isspec: ReadStorage<'a, IsSpectating>,
	pub isdead: ReadStorage<'a, IsDead>,
	pub isflag: ReadStorage<'a, IsFlag>,
}

impl<'a> System<'a> for DropOnDeath {
	type SystemData = DropOnDeathData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerKilled>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let mut channel = data.channel;

		for evt in data.commands.read(self.reader.as_mut().unwrap()) {
			let player = evt.player;

			(&*data.entities, &mut data.carrier, &data.isflag)
				.join()
				.filter(|(_, carrier, _)| carrier.0.is_some())
				.filter(|(_, carrier, _)| carrier.0.unwrap() == player)
				.for_each(|(ent, carrier, _)| {
					channel.single_write(FlagEvent {
						ty: FlagEventType::Drop,
						player: Some(player),
						flag: ent,
					});

					carrier.0 = None;
				});
		}
	}
}

impl SystemInfo for DropOnDeath {
	type Dependencies = (
		CommandHandler,
		PickupFlagSystem
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

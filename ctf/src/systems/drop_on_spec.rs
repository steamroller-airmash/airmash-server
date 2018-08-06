use specs::*;

use component::*;

use server::component::channel::*;
use server::component::flag::*;
use server::component::time::*;
use server::*;

pub struct DropOnSpec {
	pub reader: Option<OnCommandReader>,
}

#[derive(SystemData)]
pub struct DropOnSpecData<'a> {
	pub channel: Write<'a, OnFlag>,
	pub commands: Read<'a, OnCommand>,
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

impl<'a> System<'a> for DropOnSpec {
	type SystemData = DropOnSpecData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnCommand>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let mut channel = data.channel;

		for (id, packet) in data.commands.read(self.reader.as_mut().unwrap()) {
			let player = match data.conns.associated_player(*id) {
				Some(p) => p,
				None => continue,
			};

			if packet.com != "spectate" {
				continue;
			}
			let target: i32 = match packet.data.parse() {
				Ok(v) => v,
				Err(_) => continue,
			};

			match target {
				-3...-1 => (),
				_ => continue,
			}

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

impl SystemInfo for DropOnSpec {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}

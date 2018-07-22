use specs::*;

use component::*;

use server::*;

pub struct UpdateCaptures {
	reader: Option<OnFlagReader>,
}

#[derive(SystemData)]
pub struct UpdateCapturesData<'a> {
	pub channel: Read<'a, OnFlag>,
	pub conns: Read<'a, Connections>,

	pub entities: Entities<'a>,
	pub captures: WriteStorage<'a, Captures>,
}

impl UpdateCaptures {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for UpdateCaptures {
	type SystemData = UpdateCapturesData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnFlag>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			match evt.ty {
				FlagEventType::Capture => (),
				_ => continue,
			};

			let player = evt.player.unwrap();

			if !data.entities.is_alive(player) {
				continue;
			}

			data.captures.get_mut(player).unwrap().0 += 1;
		}
	}
}

impl SystemInfo for UpdateCaptures {
	// It doesn't matter too much when we handle this
	// it can happen the next frame
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}

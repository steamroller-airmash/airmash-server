use specs::*;

use component::*;

use server::component::time::ThisFrame;
use server::*;

#[derive(Default)]
pub struct UpdateLastDrop {
	reader: Option<OnFlagReader>,
}

#[derive(SystemData)]
pub struct UpdateLastDropData<'a> {
	pub channel: Read<'a, OnFlag>,

	pub lastdrop: WriteStorage<'a, LastDrop>,
	pub this_frame: Read<'a, ThisFrame>,
}

impl<'a> System<'a> for UpdateLastDrop {
	type SystemData = UpdateLastDropData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnFlag>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = match evt.ty {
				FlagEventType::Capture => None,
				FlagEventType::Drop => evt.player,
				FlagEventType::Return => None,
				_ => continue,
			};

			let lastdrop = data.lastdrop.get_mut(evt.flag).unwrap();

			*lastdrop = LastDrop {
				player: player,
				time: data.this_frame.0,
			};
		}
	}
}

impl SystemInfo for UpdateLastDrop {
	// It doesn't matter too much when we handle this
	// it can happen the next frame
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

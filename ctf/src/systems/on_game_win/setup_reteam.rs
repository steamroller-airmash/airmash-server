use specs::*;

use server::component::event::TimerEvent;
use server::types::FutureDispatcher;
use server::*;

use component::*;
use consts::RETEAM_TIMER;
use std::time::Duration;
use systems::on_flag::CheckWin;

#[derive(Default)]
pub struct SetupReteam {
	reader: Option<OnGameWinReader>,
}

#[derive(SystemData)]
pub struct SetupReteamData<'a> {
	channel: Read<'a, OnGameWin>,
	future: ReadExpect<'a, FutureDispatcher>,
}

impl<'a> System<'a> for SetupReteam {
	type SystemData = SetupReteamData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameWin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			data.future
				.run_delayed(Duration::from_secs(55), move |inst| TimerEvent {
					ty: *RETEAM_TIMER,
					instant: inst,
					data: None,
				});
		}
	}
}

impl SystemInfo for SetupReteam {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

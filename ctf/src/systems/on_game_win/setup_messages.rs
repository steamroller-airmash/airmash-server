use specs::*;

use server::component::event::TimerEvent;
use server::consts::timer::DELAYED_MESSAGE;
use server::protocol::server::ServerMessage;
use server::protocol::ServerMessageType;
use server::types::FutureDispatcher;
use server::*;

use component::*;
use config::*;
use std::time::Duration;
use systems::on_flag::CheckWin;

const MESSAGE_1_MIN: &'static str = "New game starting in 1 minute";
const MESSAGE_30_SECONDS: &'static str = "Game starting in 30 seconds - shuffling teams";
const MESSAGE_10_SECONDS: &'static str = "Game starting in 10 seconds";
const MESSAGE_5_SECONDS: &'static str = "Game starting in 5 seconds";
const MESSAGE_4_SECONDS: &'static str = "Game starting in 4 seconds";
const MESSAGE_3_SECONDS: &'static str = "Game starting in 3 seconds";
const MESSAGE_2_SECONDS: &'static str = "Game starting in 2 seconds";
const MESSAGE_1_SECONDS: &'static str = "Game starting in a second";
const MESSAGE_0_SECONDS: &'static str = "Game starting!";

const MESSAGES: [(u32, u64, &'static str); 9] = [
	(12, 60, MESSAGE_1_MIN),
	(7, 30, MESSAGE_30_SECONDS),
	(7, 10, MESSAGE_10_SECONDS),
	(2, 5, MESSAGE_5_SECONDS),
	(2, 4, MESSAGE_4_SECONDS),
	(2, 3, MESSAGE_3_SECONDS),
	(2, 2, MESSAGE_2_SECONDS),
	(2, 1, MESSAGE_1_SECONDS),
	(3, 0, MESSAGE_0_SECONDS),
];

#[derive(Default)]
pub struct SetupMessages {
	reader: Option<OnGameWinReader>,
}

#[derive(SystemData)]
pub struct SetupMessagesData<'a> {
	channel: Read<'a, OnGameWin>,
	future: ReadExpect<'a, FutureDispatcher>,
}

impl<'a> System<'a> for SetupMessages {
	type SystemData = SetupMessagesData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameWin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			for (duration, delay, msg) in MESSAGES.iter() {
				data.future.run_delayed(
					*GAME_RESET_TIME - Duration::from_secs(*delay),
					move |inst| {
						Some(TimerEvent {
							ty: *DELAYED_MESSAGE,
							instant: inst,
							data: Some(Box::new(ServerMessage {
								ty: ServerMessageType::TimeToGameStart,
								duration: *duration * 1000,
								text: msg.to_string(),
							})),
						})
					},
				);
			}
		}
	}
}

impl SystemInfo for SetupMessages {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

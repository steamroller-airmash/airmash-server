use specs::*;

use systems::PacketHandler;
use SystemInfo;

use component::channel::*;
use component::event::*;

#[derive(Default)]
pub struct ChatEventHandler {
	chat_reader: Option<OnChatReader>,
	team_reader: Option<OnTeamChatReader>,
	whisper_reader: Option<OnWhisperReader>,
	say_reader: Option<OnSayReader>,
}

#[derive(SystemData)]
pub struct ChatEventHandlerData<'a> {
	channel_chat: Read<'a, OnChat>,
	channel_team: Read<'a, OnTeamChat>,
	channel_whisper: Read<'a, OnWhisper>,
	channel_say: Read<'a, OnSay>,

	channel: Write<'a, OnAnyChatEvent>,
}

impl<'a> System<'a> for ChatEventHandler {
	type SystemData = ChatEventHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.chat_reader = Some(res.fetch_mut::<OnChat>().register_reader());
		self.team_reader = Some(res.fetch_mut::<OnTeamChat>().register_reader());
		self.whisper_reader = Some(res.fetch_mut::<OnWhisper>().register_reader());
		self.say_reader = Some(res.fetch_mut::<OnSay>().register_reader())
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel_chat.read(self.chat_reader.as_mut().unwrap()) {
			data.channel.single_write(AnyChatEvent {
				ty: ChatEventType::Public,
				text: evt.1.text.clone(),
				conn: evt.0,
			});
		}

		for evt in data.channel_team.read(self.team_reader.as_mut().unwrap()) {
			data.channel.single_write(AnyChatEvent {
				ty: ChatEventType::Team,
				text: evt.1.text.clone(),
				conn: evt.0,
			});
		}

		for evt in data.channel_say.read(self.say_reader.as_mut().unwrap()) {
			data.channel.single_write(AnyChatEvent {
				ty: ChatEventType::Say,
				text: evt.1.text.clone(),
				conn: evt.0,
			});
		}

		for evt in data
			.channel_whisper
			.read(self.whisper_reader.as_mut().unwrap())
		{
			data.channel.single_write(AnyChatEvent {
				ty: ChatEventType::Whisper(evt.1.id.0),
				text: evt.1.text.clone(),
				conn: evt.0,
			});
		}
	}
}

impl SystemInfo for ChatEventHandler {
	type Dependencies = PacketHandler;

	fn new() -> Self {
		Self::default()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

use specs::prelude::*;

use crate::systems::PacketHandler;

use crate::component::channel::*;
use crate::component::event::*;
use crate::utils::MaybeInit;

#[derive(Default)]
pub struct ChatEventHandler {
	chat_reader: MaybeInit<OnChatReader>,
	team_reader: MaybeInit<OnTeamChatReader>,
	whisper_reader: MaybeInit<OnWhisperReader>,
	say_reader: MaybeInit<OnSayReader>,
}

#[derive(SystemData, EventDeps)]
pub struct ChatEventHandlerData<'a> {
	channel_chat: Read<'a, OnChat>,
	channel_team: Read<'a, OnTeamChat>,
	channel_whisper: Read<'a, OnWhisper>,
	channel_say: Read<'a, OnSay>,

	channel: Write<'a, OnAnyChatEvent>,
}

impl<'a> System<'a> for ChatEventHandler {
	type SystemData = ChatEventHandlerData<'a>;

	fn setup(&mut self, res: &mut World) {
		Self::SystemData::setup(res);

		self.chat_reader = MaybeInit::init(res.fetch_mut::<OnChat>().register_reader());
		self.team_reader = MaybeInit::init(res.fetch_mut::<OnTeamChat>().register_reader());
		self.whisper_reader = MaybeInit::init(res.fetch_mut::<OnWhisper>().register_reader());
		self.say_reader = MaybeInit::init(res.fetch_mut::<OnSay>().register_reader())
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel_chat.read(&mut self.chat_reader) {
			data.channel.single_write(AnyChatEvent {
				ty: ChatEventType::Public,
				text: evt.1.text.clone(),
				conn: evt.0,
			});
		}

		for evt in data.channel_team.read(&mut self.team_reader) {
			data.channel.single_write(AnyChatEvent {
				ty: ChatEventType::Team,
				text: evt.1.text.clone(),
				conn: evt.0,
			});
		}

		for evt in data.channel_say.read(&mut self.say_reader) {
			data.channel.single_write(AnyChatEvent {
				ty: ChatEventType::Say,
				text: evt.1.text.clone(),
				conn: evt.0,
			});
		}

		for evt in data.channel_whisper.read(&mut self.whisper_reader) {
			data.channel.single_write(AnyChatEvent {
				ty: ChatEventType::Whisper(evt.1.id.0),
				text: evt.1.text.clone(),
				conn: evt.0,
			});
		}
	}
}

system_info! {
	impl SystemInfo for ChatEventHandler {
		type Dependencies = PacketHandler;
	}
}

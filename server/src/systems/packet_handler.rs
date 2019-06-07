use protocol::{ClientPacket, ProtocolSerializationExt};
use protocol_v5::ProtocolV5;
use shrev::EventChannel;
use specs::*;

use std::any::Any;
use std::mem;
use std::sync::mpsc::{channel, Receiver};
use std::time::Instant;

use component::channel::*;
use component::event::*;
use dispatch::*;
use types::event::*;
use types::*;

pub struct PacketHandler {
	channel: Receiver<ConnectionEvent>,
}

#[derive(SystemData)]
pub struct PacketHandlerData<'a> {
	pub onopen: Write<'a, OnOpen>,
	pub onclose: Write<'a, OnClose>,
	pub onbinary: Write<'a, OnBinary>,
	pub login: Write<'a, OnLogin>,
	pub backup: Write<'a, OnBackup>,
	pub command: Write<'a, OnCommand>,
	pub horizon: Write<'a, OnHorizon>,
	pub key: Write<'a, OnKey>,
	pub pong: Write<'a, OnPong>,
	pub say: Write<'a, OnSay>,
	pub chat: Write<'a, OnChat>,
	pub teamchat: Write<'a, OnTeamChat>,
	pub votemute: Write<'a, OnVotemute>,
	pub whisper: Write<'a, OnWhisper>,
	pub localping: Write<'a, OnLocalPing>,
	pub scoredetailed: Write<'a, OnScoreDetailed>,
	pub ack: Write<'a, OnAck>,
	pub message: Write<'a, OnMessage>,
}

impl PacketHandler {
	pub fn new(channel: Receiver<ConnectionEvent>) -> Self {
		Self { channel }
	}

	fn dispatch<'a>(
		data: &mut PacketHandlerData<'a>,
		id: ConnectionId,
		packet: ClientPacket,
		time: Instant,
	) {
		match packet {
			ClientPacket::Pong(_) => (),
			ClientPacket::Ack => (),
			_ => debug!(target:"", "Received: {:?} from {:?}", packet, id),
		}

		match packet {
			ClientPacket::Login(p) => data.login.single_write((id, p)),
			ClientPacket::Backup(p) => data.backup.single_write((id, p)),
			ClientPacket::Horizon(p) => {
				info!("{:?} {:?}", id, p);
				data.horizon.single_write((id, p));
			}
			ClientPacket::Pong(p) => data.pong.single_write(PongEvent::new(id, p, time)),
			ClientPacket::Key(p) => data.key.single_write((id, p)),
			ClientPacket::Command(p) => data.command.single_write((id, p)),
			ClientPacket::Chat(p) => data.chat.single_write((id, p)),
			ClientPacket::Whisper(p) => data.whisper.single_write((id, p)),
			ClientPacket::Say(p) => data.say.single_write((id, p)),
			ClientPacket::TeamChat(p) => data.teamchat.single_write((id, p)),
			ClientPacket::VoteMute(p) => data.votemute.single_write((id, p)),
			ClientPacket::LocalPing(p) => data.localping.single_write((id, p)),
			ClientPacket::ScoreDetailed => data.scoredetailed.single_write(ScoreDetailedEvent(id)),
			ClientPacket::Ack => data.ack.single_write(AckEvent(id)),
		}
	}
}

impl<'a> System<'a> for PacketHandler {
	type SystemData = PacketHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		// Override some default sizes
		// to prevent buffers from overflowing
		res.insert::<OnMessage>(EventChannel::with_capacity(400));
	}

	fn run(&mut self, mut sysdata: PacketHandlerData<'a>) {
		let protocol = ProtocolV5 {};
		while let Ok(evt) = self.channel.try_recv() {
			match evt {
				ConnectionEvent::ConnectionOpen(conn) => {
					sysdata.onopen.single_write(conn);
				}
				ConnectionEvent::ConnectionClose(conn) => {
					sysdata.onclose.single_write(conn);
				}
				ConnectionEvent::Message(msg) => {
					match protocol.deserialize(&msg.msg) {
						Ok(packet) => Self::dispatch(&mut sysdata, msg.conn, packet, msg.received),
						Err(_) => sysdata.onbinary.single_write((msg.conn, msg.msg.clone())),
					}

					sysdata.message.single_write(msg);
				}
			}
		}
	}
}

impl SystemInfo for PacketHandler {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		unimplemented!();
	}

	fn new_args(mut a: Box<dyn Any>) -> Self {
		let r = a.downcast_mut::<Receiver<ConnectionEvent>>().unwrap();
		Self::new(mem::replace(r, channel().1))
	}
}

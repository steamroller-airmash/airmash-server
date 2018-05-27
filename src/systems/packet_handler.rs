
use specs::*;
use specs::prelude::*;
use shrev::EventChannel;
use websocket::OwnedMessage;
use airmash_protocol::from_bytes;
use airmash_protocol::client::*;

use std::sync::mpsc::{Receiver, Sender, channel};

use types::*;

pub struct PacketHandler {
	channel: Receiver<ConnectionEvent>
}

#[derive(SystemData)]
pub struct PacketHandlerData<'a> {
		pub onopen:        Write<'a, EventChannel<ConnectionOpen>>,
		pub onclose:       Write<'a, EventChannel<ConnectionClose>>,
		pub onbinary:      Write<'a, EventChannel<Message>>,
		pub login:         Write<'a, EventChannel<(ConnectionId, Login)>>,
		pub backup:        Write<'a, EventChannel<(ConnectionId, Backup)>>,
		pub command:       Write<'a, EventChannel<(ConnectionId, Command)>>,
		pub horizon:       Write<'a, EventChannel<(ConnectionId, Horizon)>>,
		pub key:           Write<'a, EventChannel<(ConnectionId, Key)>>,
		pub pong:          Write<'a, EventChannel<(ConnectionId, Pong)>>,
		pub say:           Write<'a, EventChannel<(ConnectionId, Say)>>,
		pub chat:          Write<'a, EventChannel<(ConnectionId, Chat)>>,
		pub teamchat:      Write<'a, EventChannel<(ConnectionId, TeamChat)>>,
		pub votemute:      Write<'a, EventChannel<(ConnectionId, VoteMute)>>,
		pub whisper:       Write<'a, EventChannel<(ConnectionId, Whisper)>>,
		pub localping:     Write<'a, EventChannel<(ConnectionId, LocalPing)>>,
		pub scoredetailed: Write<'a, EventChannel<ConnectionId>>,
		pub ack:           Write<'a, EventChannel<ConnectionId>>,
}

impl PacketHandler {
	pub fn new(channel: Receiver<ConnectionEvent>) -> Self {
		Self { 
			channel
		}
	}

	fn dispatch<'a>(
		data: &mut PacketHandlerData<'a>, 
		id: ConnectionId,
		packet: ClientPacket
	) {
		match packet {
			ClientPacket::Login(p) =>     data.login.single_write((id, p) ),
			ClientPacket::Backup(p) =>    data.backup.single_write((id, p)),
			ClientPacket::Horizon(p) =>   data.horizon.single_write((id, p)),
			ClientPacket::Pong(p) =>      data.pong.single_write((id, p)),
			ClientPacket::Key(p) =>       data.key.single_write((id, p)),
			ClientPacket::Command(p) =>   data.command.single_write((id, p)),
			ClientPacket::Chat(p) =>      data.chat.single_write((id, p)),
			ClientPacket::Whisper(p) =>   data.whisper.single_write((id, p)),
			ClientPacket::Say(p) =>       data.say.single_write((id, p)),
			ClientPacket::TeamChat(p) =>  data.teamchat.single_write((id, p)),
			ClientPacket::VoteMute(p) =>  data.votemute.single_write((id, p)),
			ClientPacket::LocalPing(p) => data.localping.single_write((id, p)),
			ClientPacket::ScoreDetailed=> data.scoredetailed.single_write(id),
			ClientPacket::Ack =>          data.ack.single_write(id)
		}
	}
}

impl<'a> System<'a> for PacketHandler {
	type SystemData = PacketHandlerData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		// Override some default sizes
		// to prevent buffers from overflowing
		res.insert::<EventChannel<(ConnectionId, Pong)>>(
			EventChannel::with_capacity(200)
		);
		res.insert::<EventChannel<Message>>(
			EventChannel::with_capacity(200)
		);
	}

	fn run(&mut self, mut sysdata: PacketHandlerData<'a>) {
		while let Ok(evt) = self.channel.try_recv() {
			match evt {
				ConnectionEvent::ConnectionOpen(conn) => {
					sysdata.onopen.single_write(conn);
				},
				ConnectionEvent::ConnectionClose(conn) => {
					sysdata.onclose.single_write(conn);
				},
				ConnectionEvent::Message(msg) => {
					if let OwnedMessage::Binary(data) = msg.msg {
						match from_bytes::<ClientPacket>(&data) {
							Ok(packet) => Self::dispatch(&mut sysdata, msg.conn, packet),
							Err(_) => sysdata.onbinary.single_write(Message { 
								conn: msg.conn, 
								msg: OwnedMessage::Binary(data) 
							})
						}
					}
				}
			}
		}
	}
}


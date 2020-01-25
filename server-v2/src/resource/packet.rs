use shrev::EventChannel;

use crate::resource::socket::SocketId;

pub struct ClientPacket<P> {
    pub connection: SocketId,
    pub packet: P,
}

// So that we can avoid formatting here
#[rustfmt::skip]
mod inner {
	use airmash_protocol::client::*;
	use super::*;

	pub type OnAck      	 = EventChannel<ClientPacket<Ack>>;
	pub type OnBackup    	 = EventChannel<ClientPacket<Backup<'static>>>;
	pub type OnChat      	 = EventChannel<ClientPacket<Chat<'static>>>;
	pub type OnCommand   	 = EventChannel<ClientPacket<Command<'static>>>;
	pub type OnHorizon   	 = EventChannel<ClientPacket<Horizon>>;
	pub type OnKey			 = EventChannel<ClientPacket<Key>>;
	pub type OnLocalPing	 = EventChannel<ClientPacket<LocalPing>>;
	pub type OnLogin		 = EventChannel<ClientPacket<Login<'static>>>;
	pub type OnPong			 = EventChannel<ClientPacket<Pong>>;
	pub type OnSay			 = EventChannel<ClientPacket<Say<'static>>>;
	pub type OnScoreDetailed = EventChannel<ClientPacket<ScoreDetailed>>;
	pub type OnTeamChat		 = EventChannel<ClientPacket<TeamChat<'static>>>;
	pub type OnVoteMute		 = EventChannel<ClientPacket<VoteMute>>;
	pub type OnWhisper		 = EventChannel<ClientPacket<Whisper<'static>>>;
}

pub use self::inner::*;

/// Happens when an unparseable packet is recieved.
pub type OnUnknown = EventChannel<ClientPacket<Vec<u8>>>;

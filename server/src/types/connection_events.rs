use std::net::IpAddr;
use std::sync::Mutex;

use types::{ConnectionId, ConnectionSink};
use websocket::OwnedMessage;

pub struct ConnectionOpen {
	pub conn: ConnectionId,
	pub sink: Mutex<Option<ConnectionSink>>,
	pub addr: IpAddr,
	pub origin: Option<String>,
}

#[derive(Copy, Clone, Debug)]
pub struct ConnectionClose {
	pub conn: ConnectionId,
}

#[derive(Clone, Debug)]
pub struct Message {
	pub conn: ConnectionId,
	pub msg: OwnedMessage,
}

pub enum ConnectionEvent {
	ConnectionOpen(ConnectionOpen),
	ConnectionClose(ConnectionClose),
	Message(Message),
}


use types::{ConnectionId, ConnectionSink};
use websocket::OwnedMessage;
use std::sync::Mutex;

pub struct ConnectionOpen {
	pub conn: ConnectionId,
	pub sink: Mutex<Option<ConnectionSink>>
}

#[derive(Copy, Clone, Debug)]
pub struct ConnectionClose {
	pub conn: ConnectionId
}

#[derive(Clone, Debug)]
pub struct Message {
	pub conn: ConnectionId,
	pub msg: OwnedMessage
}

pub enum ConnectionEvent {
	ConnectionOpen(ConnectionOpen),
	ConnectionClose(ConnectionClose),
	Message(Message)
}
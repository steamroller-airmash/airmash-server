use std::net::IpAddr;
use std::time::Instant;
use types::ConnectionId;

use ws::Sender as WsSender;

pub struct ConnectionOpen {
	pub conn: ConnectionId,
	pub sink: WsSender,
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
	pub received: Instant,
	pub msg: Vec<u8>,
}

pub enum ConnectionEvent {
	ConnectionOpen(ConnectionOpen),
	ConnectionClose(ConnectionClose),
	Message(Message),
}

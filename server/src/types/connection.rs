use fnv::FnvHashMap;
use specs::Entity;

use std::net::IpAddr;

use protocol::{Protocol, ServerPacket};
use protocol_v5::ProtocolV5;
use types::ConnectionId;

use ws::{self, Sender as WsSender};

pub struct ConnectionData {
	pub sink: WsSender,
	pub id: ConnectionId,
	pub ty: ConnectionType,
	pub player: Option<Entity>,
	pub info: ConnectionInfo,
}

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
	pub addr: IpAddr,
	pub origin: Option<String>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ConnectionType {
	Primary,
	Backup,
	Inactive,
}

pub struct Connections {
	pub conns: FnvHashMap<ConnectionId, ConnectionData>,
}

impl Default for Connections {
	fn default() -> Self {
		Connections::new()
	}
}

impl Connections {
	pub fn new() -> Self {
		Connections {
			conns: FnvHashMap::default(),
		}
	}

	pub fn add(&mut self, id: ConnectionId, sink: WsSender, addr: IpAddr, origin: Option<String>) {
		let data = ConnectionData {
			sink: sink,
			ty: ConnectionType::Inactive,
			player: None,
			id: id,
			info: ConnectionInfo { addr, origin },
		};

		self.conns.insert(id, data);
	}
	pub fn remove(&mut self, id: ConnectionId) {
		self.conns.remove(&id).unwrap_or_else(|| {
			error!(
				target: "server",
				"Attempted to remove non-existent connection {:?}",
				id
			);
			panic!("Nonexistent connection id {:?}", id);
		});
	}
	pub fn remove_player(&mut self, player: Entity) {
		let mut conns = vec![];

		for conn in self.conns.values() {
			if let Some(p) = conn.player {
				if p == player {
					conns.push(conn.id);
				}
			}
		}

		for id in conns {
			self.remove(id);
		}
	}

	pub fn associate(&mut self, id: ConnectionId, player: Entity, ty: ConnectionType) {
		let ref mut conn = self.conns.get_mut(&id).unwrap_or_else(|| {
			error!(
				target: "server",
				"Attempted to associate non-existent connection {:?} with player {:?}",
				id, player
			);
			panic!("Nonexistent connection id {:?}", id);
		});

		conn.player = Some(player);
		conn.ty = ty;
	}

	pub fn send_sink(conn: &mut WsSender, msg: ws::Message) {
		conn.send(msg).unwrap();
	}

	pub fn send_to<I>(&self, id: ConnectionId, msg: I)
	where
		I: Into<ServerPacket>,
	{
		self.send_to_ref(id, &msg.into())
	}
	pub fn send_to_ref(&self, id: ConnectionId, msg: &ServerPacket) {
		// FIXME: Send errors back up to the caller
		trace!(
			target: "server",
			"Sent message to {:?}: {:?}",
			id, msg
		);

		let mut conn = match self.conns.get(&id).map(|ref x| x.sink.clone()) {
			Some(conn) => conn,
			None => {
				// The connection probably closed, do nothing
				trace!(
					target: "server",
					"Tried to send message to closed connection {:?}",
					id
				);
				return;
			}
		};

		let protocol = ProtocolV5 {};

		let serialized = match protocol.serialize_server(&msg) {
			Ok(x) => x,
			Err(e) => {
				warn!(
					"Serialization error while sending a packet:\n{}\nPacket data was:\n{:#?}",
					e, msg
				);
				return;
			}
		};

		for data in serialized {
			Self::send_sink(&mut conn, ws::Message::Binary(data));
		}
	}

	pub fn close(&self, id: ConnectionId) {
		use ws::CloseCode;

		let conn = match self.conns.get(&id).map(|x| x.sink.clone()) {
			Some(conn) => conn,
			None => {
				trace!(
					target: "server",
					"Tried to close an already closed connection: {:?}",
					id
				);
				return;
			}
		};

		conn.close(CloseCode::Normal).unwrap();
	}

	pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a ConnectionData> {
		self.conns.values()
	}

	pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut ConnectionData> {
		self.conns.values_mut()
	}

	pub fn associated_player(&self, connid: ConnectionId) -> Option<Entity> {
		match self.conns.get(&connid) {
			Some(ref v) => v.player,
			None => None,
		}
	}
}

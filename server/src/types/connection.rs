use types::ConnectionId;

use fnv::FnvHashMap;
use futures::stream::SplitSink;
use futures::{AsyncSink, Sink};
use specs::Entity;
use websocket::async::{MessageCodec, TcpStream};
// Can't change this yet since websocket has not updated
#[allow(deprecated)]
use websocket::client::async::Framed;
use websocket::OwnedMessage;

use std::net::IpAddr;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

// Websocket hasn't updated, can't change this yet
#[allow(deprecated)]
pub type ConnectionSink = SplitSink<Framed<TcpStream, MessageCodec<OwnedMessage>>>;

pub struct ConnectionData {
	pub sink: ConnectionSink,
	pub id: ConnectionId,
	pub ty: ConnectionType,
	pub player: Option<Entity>,
	pub addr: IpAddr,
	pub origin: Option<String>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ConnectionType {
	Primary,
	Backup,
	Inactive,
}

pub enum MessageInfo {
	ToConnection(ConnectionId),
	ToTeam(Entity),
	ToVisible(Entity),
}

pub struct Message {
	pub info: MessageInfo,
	pub msg: OwnedMessage,
}

pub struct Connections(
	pub FnvHashMap<ConnectionId, ConnectionData>,
	Mutex<Sender<Message>>,
);

impl Default for Connections {
	fn default() -> Self {
		panic!("No default for connections");
	}
}

impl Connections {
	pub fn new(channel: Sender<Message>) -> Self {
		Connections(FnvHashMap::default(), Mutex::new(channel))
	}

	pub fn add(
		&mut self,
		id: ConnectionId,
		sink: ConnectionSink,
		addr: IpAddr,
		origin: Option<String>,
	) {
		let data = ConnectionData {
			sink: sink,
			ty: ConnectionType::Inactive,
			player: None,
			id: id,
			addr,
			origin,
		};

		self.0.insert(id, data);
	}
	pub fn remove(&mut self, id: ConnectionId) {
		self.0.remove(&id).unwrap_or_else(|| {
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

		for conn in self.0.values() {
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
		let ref mut conn = self.0.get_mut(&id).unwrap_or_else(|| {
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

	pub fn send_sink(conn: &mut ConnectionSink, msg: OwnedMessage) {
		conn.start_send(msg)
			.and_then(|x| {
				match x {
					AsyncSink::Ready => (),
					AsyncSink::NotReady(item) => {
						conn.poll_complete().unwrap();
						conn.start_send(item).unwrap();
					}
				}
				Ok(())
			})
			.unwrap();
	}

	pub fn send_to_player(&self, player: Entity, msg: OwnedMessage) {
		let conn = self.0.iter().find(|(_, c)| {
			c.player.is_some() && c.ty == ConnectionType::Primary && c.player.unwrap() == player
		});

		if conn.is_none() {
			warn!(
				target: "server",
				"Attempted to send message to nonexistent player {:?}",
				player
			);

			return;
		}

		self.send_to(*conn.unwrap().0, msg);
	}

	pub fn send_to(&self, id: ConnectionId, msg: OwnedMessage) {
		trace!(
			target: "server",
			"Sent message to {:?}: {:?}",
			id, msg
		);

		self.1
			.lock()
			.unwrap()
			.send(Message {
				info: MessageInfo::ToConnection(id),
				msg: msg,
			})
			.unwrap();
	}

	pub fn send_to_all(&self, msg: OwnedMessage) {
		self.0
			.iter()
			.filter_map(|(id, ref conn)| {
				if conn.player.is_some() {
					if conn.ty == ConnectionType::Primary {
						return Some(id);
					}
				}
				None
			})
			.for_each(|id| {
				self.1
					.lock()
					.unwrap()
					.send(Message {
						info: MessageInfo::ToConnection(*id),
						msg: msg.clone(),
					})
					.unwrap();
			});
	}

	pub fn send_to_others(&self, player: Entity, msg: OwnedMessage) {
		self.0
			.iter()
			.filter_map(|(id, ref conn)| {
				if let Some(ent) = conn.player {
					if conn.ty == ConnectionType::Primary && ent != player {
						return Some(id);
					}
				}
				None
			})
			.for_each(|id| {
				self.1
					.lock()
					.unwrap()
					.send(Message {
						info: MessageInfo::ToConnection(*id),
						msg: msg.clone(),
					})
					.unwrap()
			});
	}

	pub fn send_to_team(&self, player: Entity, msg: OwnedMessage) {
		self.1
			.lock()
			.unwrap()
			.send(Message {
				info: MessageInfo::ToTeam(player),
				msg: msg,
			})
			.unwrap();
	}

	pub fn send_to_visible(&self, player: Entity, msg: OwnedMessage) {
		self.1
			.lock()
			.unwrap()
			.send(Message {
				info: MessageInfo::ToVisible(player),
				msg: msg,
			})
			.unwrap();
	}

	pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a ConnectionData> {
		self.0.values()
	}

	pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut ConnectionData> {
		self.0.values_mut()
	}

	pub fn associated_player(&self, connid: ConnectionId) -> Option<Entity> {
		match self.0.get(&connid) {
			Some(ref v) => v.player,
			None => None,
		}
	}
}

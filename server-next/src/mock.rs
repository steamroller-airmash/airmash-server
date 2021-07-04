use crate::network::*;
use crate::protocol::{client as c, ClientPacket, ServerPacket};

use airmash_protocol::KeyCode;
use crossbeam_channel::Sender;
use std::net::{IpAddr, SocketAddr};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

/// Mock connection for testing purposes.
///
/// This allows for sending packets to the server without having to go through a
/// network connection. This is essential for ensuring that tests don't
/// interfere with each other.
pub struct MockConnection {
  tx: Sender<(ConnectionId, InternalEvent)>,
  rx: UnboundedReceiver<Vec<u8>>,
  conn: ConnectionId,
  closed: bool,

  seq: u32,
}

impl MockConnection {
  fn new(
    tx: Sender<(ConnectionId, InternalEvent)>,
    rx: UnboundedReceiver<Vec<u8>>,
    conn: ConnectionId,
  ) -> Self {
    Self {
      tx,
      rx,
      conn,
      closed: false,

      seq: 0,
    }
  }

  pub fn conn(&self) -> ConnectionId {
    self.conn
  }

  pub fn next_raw(&mut self) -> Option<Vec<u8>> {
    use std::task::{Context, Poll};

    let waker = futures_util::task::noop_waker_ref();
    let mut ctx = Context::from_waker(waker);
    match self.rx.poll_recv(&mut ctx) {
      Poll::Pending => None,
      Poll::Ready(x) => x,
    }
  }

  pub fn next_packet(&mut self) -> Option<ServerPacket> {
    let data = self.next_raw()?;
    let packet = crate::protocol::v5::deserialize(&data).unwrap_or_else(|e| {
      panic!(
        "Server sent invalid packet. Error is {}. Packet is:\n  {:?}",
        e, data
      )
    });

    println!("{}: {:#?}", self.conn.id(), packet);

    Some(packet)
  }

  pub fn send_raw(&mut self, data: Vec<u8>) {
    assert!(!self.closed, "Tried to send to a closed client");

    self
      .tx
      .send((self.conn, InternalEvent::Data(data)))
      .expect("Network event channel is closed");
  }

  pub fn send(&mut self, packet: impl Into<ClientPacket>) {
    let data = crate::protocol::v5::serialize(&packet.into()).expect("Failed to serialize packet");
    self.send_raw(data);
  }

  pub fn close(&mut self) {
    if std::mem::replace(&mut self.closed, false) {
      let _ = self.tx.send((self.conn, InternalEvent::Closed));
    }
  }
}

impl MockConnection {
  pub fn send_key(&mut self, key: KeyCode, state: bool) {
    self.send(c::Key {
      key,
      state,
      seq: self.seq,
    });

    self.seq += 1;
  }
}

impl Drop for MockConnection {
  fn drop(&mut self) {
    self.close();
  }
}

/// Mock connection endpoint for testing purposes.
///
/// This allows for creating new connections without needing open an actual
/// server on a socket.
pub struct MockConnectionEndpoint {
  sender: Sender<(ConnectionId, InternalEvent)>,
  nextid: usize,
}

impl MockConnectionEndpoint {
  pub(crate) fn new(sender: Sender<(ConnectionId, InternalEvent)>) -> Self {
    Self { sender, nextid: 0 }
  }

  pub fn open(&mut self) -> MockConnection {
    let conn = ConnectionId(self.nextid);
    self.nextid += 1;

    let (tx, rx) = unbounded_channel();

    self
      .sender
      .send((
        conn,
        InternalEvent::Opened(ConnectionData {
          send: tx,
          addr: SocketAddr::new(IpAddr::from([0; 4]), 0),
        }),
      ))
      .expect("Network event channel is closed");

    MockConnection::new(self.sender.clone(), rx, conn)
  }
}

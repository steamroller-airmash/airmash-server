//! Event channels related to new packets, socket connections,
//! and socket disconnects.

use shrev::EventChannel;
use tokio::sync::mpsc::UnboundedSender;

pub type OnConnect = EventChannel<ConnectEvent>;
pub type OnMessage = EventChannel<MessageEvent>;
pub type OnClose = EventChannel<CloseEvent>;

/// Event for when a new connection is established.
///
/// This event does not mean that a new player has
/// logged on.
#[allow(dead_code)]
pub struct ConnectEvent {
    pub(crate) origin: Option<Vec<u8>>,
    pub(crate) forwarded: Option<Vec<u8>>,
    pub(crate) x_forwarded_for: Option<Vec<u8>>,
    pub(crate) sec_websocket_protocol: Option<Vec<u8>>,
    pub(crate) sec_websocket_version: Option<u32>,

    pub socket: SocketWriter,
    pub socketid: SocketId,
}

/// Event for when a message is received in a packet.
#[derive(Clone, Debug)]
pub struct MessageEvent {
    pub socket: SocketId,
    pub data: Vec<u8>,
}

/// Event for when a connection is closed.
#[derive(Clone, Debug)]
pub struct CloseEvent {
    pub socket: SocketId,
}

/// A identifier that uniquely identifies a socket.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SocketId(u64);

#[allow(clippy::new_without_default)]
impl SocketId {
    /// Create a new unique socket id.
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static IDCTR: AtomicU64 = AtomicU64::new(0);

        Self(IDCTR.fetch_add(1, Ordering::Relaxed))
    }
}

impl ConnectEvent {
    pub fn remote_addr(&self) -> Option<&[u8]> {
        unimplemented!()
    }
}

pub struct SocketWriter {
    sender: UnboundedSender<SocketMessage>,
}

pub struct SendError(());

impl SocketWriter {
    #[doc(hidden)]
    pub(crate) fn new(sender: UnboundedSender<SocketMessage>) -> Self {
        Self { sender }
    }

    pub fn write(&mut self, buf: Vec<u8>) -> Result<(), SendError> {
        self.sender
            .send(SocketMessage::Data(buf))
            .map_err(|_| SendError(()))
    }

    pub fn close(self) {
        let _ = self.sender.send(SocketMessage::Close);
    }
}

#[derive(Clone, Debug)]
pub(crate) enum SocketMessage {
    Data(Vec<u8>),
    Close,
}

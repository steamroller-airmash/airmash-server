use fxhash::FxHashMap as HashMap;

use std::fmt;

use crate::ecs::Entity;
use crate::resource::socket::{SocketId, SocketWriter};
use crate::util::RcBuf;

struct ConnInfo {
    writer: SocketWriter,
    player: Option<Entity>,
}

/// Resource which maps socket ids to socket writers and
/// also associates players with sockets.
#[derive(Default)]
pub struct Connections {
    conns: HashMap<SocketId, ConnInfo>,
}

impl Connections {
    /// Register a newly created socket into this resource.
    pub fn register_new(&mut self, socket: SocketId, writer: SocketWriter) {
        self.conns.insert(
            socket,
            ConnInfo {
                writer,
                player: None,
            },
        );
    }

    /// Associate a player with an existing socket.
    pub fn associate(
        &mut self,
        socket: SocketId,
        player: Entity,
    ) -> Result<(), NonexistantSocketError> {
        match self.conns.get_mut(&socket) {
            Some(info) => {
                info.player = Some(player);
                Ok(())
            }
            None => Err(NonexistantSocketError(socket)),
        }
    }

    /// Get the player associated with a `SocketId`.
    pub fn player(&self, socket: SocketId) -> Result<Option<Entity>, NonexistantSocketError> {
        match self.conns.get(&socket) {
            Some(info) => Ok(info.player),
            None => Err(NonexistantSocketError(socket)),
        }
    }

    /// Get the `SocketWriter` associated with a `SocketId`.
    pub fn writer(&self, socket: SocketId) -> Result<&SocketWriter, NonexistantSocketError> {
        match self.conns.get(&socket) {
            Some(info) => Ok(&info.writer),
            None => Err(NonexistantSocketError(socket)),
        }
    }

    /// Send a message to a socket. Returns whether or not
    /// sending succeeded.
    ///
    /// Sending can fail if the other end of the socket has
    /// hung up.
    pub fn send_to(
        &self,
        socket: SocketId,
        data: impl Into<RcBuf<u8>>,
    ) -> Result<bool, NonexistantSocketError> {
        Ok(self.writer(socket)?.write(data.into()).is_ok())
    }

    /// Close an open socket.
    pub fn close(&mut self, socket: SocketId) -> Result<(), NonexistantSocketError> {
        match self.conns.remove(&socket) {
            Some(info) => {
                info.writer.close();
                Ok(())
            }
            None => Err(NonexistantSocketError(socket)),
        }
    }
}

/// Error for when a `SocketId` referring to a socket
/// not stored within the `Connections` resource is used.
#[derive(Debug)]
pub struct NonexistantSocketError(SocketId);

impl fmt::Display for NonexistantSocketError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Socket {} does not exist", self.0)
    }
}

impl std::error::Error for NonexistantSocketError {}

//! Infrastructure related to starting up the server.

mod config;
mod server;
mod timeloop;
mod wshandler;

pub use self::config::AirmashServerConfig;
pub use self::server::AirmashServer;

pub(self) use self::wshandler::spawn_acceptor;

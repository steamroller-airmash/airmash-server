//! Custom SystemData objects that are used within the server.

mod clock;
mod connections;
mod task_spawner;

pub use crate::util::GameModeWriter;

pub use self::clock::ReadClock;
pub use self::connections::{Connections, ConnectionsMut, ConnectionsNoTeams};
pub use self::task_spawner::{TaskData, TaskSpawner};

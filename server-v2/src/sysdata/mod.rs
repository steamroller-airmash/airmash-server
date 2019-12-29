//! Custom SystemData objects that are used within the server.

mod task_spawner;

pub use crate::util::GameModeWriter;

pub use self::task_spawner::{TaskSpawner, TaskData};

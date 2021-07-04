//! All resource types used within the server.

use bstr::BString;
use hecs::Entity;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub mod collision;
pub mod config;

mod game_config;
mod tasks;

pub use self::config::Config;
pub use self::game_config::GameConfig;
pub use self::tasks::{Task, TaskScheduler};
pub use crate::protocol::GameType;

def_wrappers! {
  /// Time at which the last frame occurred.
  ///
  /// This can also be accessed via [`AirmashGame::last_frame`].
  ///
  /// [`AirmashGame::last_frame`]: crate::AirmashGame::last_frame
  pub type LastFrame = Instant;

  /// Time at which the current frame is occurring.
  ///
  /// This can also be accessed via [`AirmashGame::this_frame`].
  ///
  /// [`AirmashGame::this_frame`]: crate::AirmashGame::this_frame
  pub type ThisFrame = Instant;

  /// Time at which the server is started.
  ///
  /// This also useful as a time that is before any possible value of
  /// [`ThisFrame`].
  pub type StartTime = Instant;
}

def_wrapper_resources! {
  /// The name of the current region.
  ///
  /// This is what is shown to the player in the menu on the top left where
  /// they have the option to change servers.
  ##[nocopy]
  pub type RegionName = String;

  /// Record of the names of players currently within the server.
  ///
  /// This is used to avoid assigning the same name to multiple players when
  /// a new player attempts to log in with an existing name.
  ##[nocopy]
  pub type TakenNames = HashSet<BString>;

  /// Mapping of user-facing ID to existing entities.
  ##[nocopy]
  pub type EntityMapping = HashMap<u16, Entity>;
}

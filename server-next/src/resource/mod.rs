use bstr::BString;
use hecs::Entity;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub mod collision;
mod config;
mod tasks;
mod game_config;

pub use self::config::*;
pub use self::tasks::{TaskScheduler, Task};
pub use self::game_config::GameConfig;

def_wrappers! {
  pub type LastFrame = Instant;
  pub type ThisFrame = Instant;
  pub type StartTime = Instant;
}

// trace_macros!(true);
def_wrapper_resources! {
  ##[nocopy]
  pub type GameRoom = String;

  ##[nocopy]
  pub type TakenNames = HashSet<BString>;

  ##[nocopy]
  pub type EntityMapping = HashMap<u16, Entity>;
}
// trace_macros!(false);


use std::time::Instant;
use std::collections::HashSet;
use bstr::BString;

mod config;
pub mod collision;

pub use self::config::*;

def_wrappers! {
  pub type LastFrame = Instant;
  pub type ThisFrame = Instant;
  pub type StartTime = Instant;

  ##[nocopy]
  pub type GameRoom = String;
  
  ##[nocopy]
  pub type TakenNames = HashSet<BString>;
}

use super::Effects;
use crate::component::SpecialActive;
use crate::config::PlanePrototypeRef;
use crate::protocol::ServerKeyState;

/// Known key state of a player.
///
/// This is kept updated based on packets from the client. However, if the
/// client is dead or recently respawned it may be innaccurate. This ends up
/// being corrected the first time the player presses a key.
#[derive(Default, Clone, Debug)]
pub struct KeyState {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  pub fire: bool,
  pub special: bool,
  // This might not be the best place to
  // keep these, can be moved later if
  // necessary
  pub stealthed: bool,
  pub flagspeed: bool,
}

impl KeyState {
  pub fn strafe(&self, plane: &PlanePrototypeRef) -> bool {
    plane.special.is_strafe() && self.special
  }

  pub fn to_server(
    &self,
    plane: &PlanePrototypeRef,
    active: &SpecialActive,
    _effects: &Effects,
  ) -> ServerKeyState {
    ServerKeyState {
      up: self.up,
      down: self.down,
      left: self.left,
      right: self.right,
      boost: plane.special.is_boost() && active.0,
      strafe: plane.special.is_strafe() && self.special,
      stealth: plane.special.is_stealth() && active.0,
      flagspeed: self.flagspeed,
    }
  }
}

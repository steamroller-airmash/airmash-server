use crate::protocol::{PlaneType, ServerKeyState};
use crate::component::SpecialActive;

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
  pub fn strafe(&self, plane: &PlaneType) -> bool {
    *plane == PlaneType::Mohawk && self.special
  }

  pub fn to_server(&self, plane: &PlaneType, active: &SpecialActive) -> ServerKeyState {
    ServerKeyState {
      up: self.up,
      down: self.down,
      left: self.left,
      right: self.right,
      boost: *plane == PlaneType::Predator && active.0,
      strafe: self.strafe(plane),
      stealth: self.stealthed,
      flagspeed: self.flagspeed,
    }
  }
}

use crate::protocol::{PlaneType, ServerKeyState};

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
  pub fn boost(&self, plane: &PlaneType) -> bool {
    *plane == PlaneType::Predator && self.special && (self.up || self.down)
  }
  pub fn strafe(&self, plane: &PlaneType) -> bool {
    *plane == PlaneType::Mohawk && self.special
  }

  pub fn to_server(&self, plane: &PlaneType) -> ServerKeyState {
    ServerKeyState {
      up: self.up,
      down: self.down,
      left: self.left,
      right: self.right,
      boost: self.boost(plane),
      strafe: self.strafe(plane),
      stealth: self.stealthed,
      flagspeed: self.flagspeed,
    }
  }
}

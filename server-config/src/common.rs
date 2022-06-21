use std::borrow::Cow;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::util::duration;
use crate::{PlanePrototype, PrototypeRef, PtrRef, StringRef, ValidationError};

/// Common fields that are just copied directly between [`GamePrototype`] and
/// [`GameConfig`].
///
/// [`GamePrototype`]: crate::GamePrototype
/// [`GameConfig`]: crate::GameConfig
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(bound(
  serialize = "Ref::PlaneRef: Serialize",
  deserialize = "Ref::PlaneRef: Deserialize<'de>"
))]
pub struct GameConfigCommon<'a, Ref: PrototypeRef<'a>> {
  /// The default plane that a player joining the game will get unless the
  /// server overrides it.
  pub default_plane: Ref::PlaneRef,

  /// The radius in which the player can observe events happening.
  pub view_radius: f32,

  #[serde(with = "duration")]
  pub respawn_delay: Duration,
}

impl GameConfigCommon<'_, StringRef> {
  pub const fn new() -> Self {
    Self {
      default_plane: Cow::Borrowed("predator"),
      view_radius: 2250.0,
      respawn_delay: Duration::from_secs(2),
    }
  }

  pub(crate) fn resolve<'a>(
    self,
    planes: &'a [PlanePrototype<'a, PtrRef>],
  ) -> Result<GameConfigCommon<'a, PtrRef>, ValidationError> {
    let default_plane =
      planes
        .iter()
        .find(|p| p.name == self.default_plane)
        .ok_or(ValidationError::custom(
          "default_plane",
          format_args!(
            "default_plane refers to a plane prototype `{}` which does not exist",
            self.default_plane
          ),
        ))?;

    Ok(GameConfigCommon {
      default_plane,
      view_radius: self.view_radius,
      respawn_delay: self.respawn_delay,
    })
  }
}

impl Default for GameConfigCommon<'_, StringRef> {
  fn default() -> Self {
    Self::new()
  }
}

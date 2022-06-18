use std::borrow::Cow;

use serde::{Deserialize, Serialize};

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
}

impl GameConfigCommon<'_, StringRef> {
  pub const fn new() -> Self {
    Self {
      default_plane: Cow::Borrowed("predator"),
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

    Ok(GameConfigCommon { default_plane })
  }
}

impl Default for GameConfigCommon<'_, StringRef> {
  fn default() -> Self {
    Self::new()
  }
}

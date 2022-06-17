use std::borrow::Cow;

/// Common fields that are just copied directly between [`GamePrototype`] and
/// [`GameConfig`].
///
/// [`GamePrototype`]: crate::GamePrototype
/// [`GameConfig`]: crate::GameConfig
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameConfigCommon {
  /// The default plane that a player joining the game will get unless the
  /// server overrides it.
  pub default_plane: Cow<'static, str>,
}

impl GameConfigCommon {
  pub const fn new() -> Self {
    Self {
      default_plane: Cow::Borrowed("predator"),
    }
  }
}

impl Default for GameConfigCommon {
  fn default() -> Self {
    Self::new()
  }
}

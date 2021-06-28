/// Flags to enable and/or disable engine features.
///
/// By default these configs are set as would be needed for an FFA gamemode.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameConfig {
  /// Whether or not to enable the default respawn logic.
  ///
  /// If this is set to true then, by default, once a player dies then they will
  /// be automatically respawned in 2 seconds unless they decide to spectate.
  /// If this is set to false then dead players will be unable to respawn until
  /// some external server logic allows them to.
  ///
  /// By default this is set to `true`.
  pub default_respawn: bool,
}

impl Default for GameConfig {
  fn default() -> Self {
    Self {
      default_respawn: true
    }
  }
}

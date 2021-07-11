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
  /// This is enabled by default
  pub default_respawn: bool,

  /// Whether or not players are allowed to respawn explicitly.
  ///
  /// If this is set to true then a player can request a respawn whenever they
  /// are inactive. This will also allow them to respawn with a new plane. If
  /// false then respawn requests are ignored.
  ///
  /// This is enabled by default.
  pub allow_respawn: bool,

  /// Whether or not players can damage each other.
  /// 
  /// This is set to true by default.
  pub allow_damage: bool
}

impl Default for GameConfig {
  fn default() -> Self {
    Self {
      default_respawn: true,
      allow_respawn: true,
      allow_damage: true
    }
  }
}

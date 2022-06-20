use std::ops::Deref;

/// Flags to enable and/or disable engine features.
///
/// By default these configs are set as would be needed for an FFA gamemode.
#[derive(Clone, Debug)]
pub struct GameConfig {
  /// Whether or not to enable the default respawn logic.
  ///
  /// If this is set to true then, by default, once a player dies then they will
  /// be automatically respawned in 2 seconds unless they decide to spectate.
  /// If this is set to false then dead players will be unable to respawn until
  /// some external server logic allows them to.
  ///
  /// This is enabled by default.
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
  pub allow_damage: bool,

  /// Whether players will occasionally drop upgrades on death. If this is set
  /// to false then no upgrades will ever drop.
  ///
  /// This is set to true by default.
  pub spawn_upgrades: bool,

  /// Whether to default players to always being fully upgraded. If set to true
  /// then players will always have 5555 upgrades.
  ///
  /// This is set to false by default.
  pub always_upgraded: bool,

  /// Whether admin commands are enabled.
  ///
  /// This is set to false by default.
  ///
  /// TODO: This should be replaced with authenticating for admin commands.
  pub admin_enabled: bool,

  pub inner: server_config::GameConfig,
}

impl Default for GameConfig {
  fn default() -> Self {
    Self {
      default_respawn: true,
      allow_respawn: true,
      allow_damage: true,
      spawn_upgrades: true,
      always_upgraded: false,
      admin_enabled: false,
      inner: server_config::GameConfig::default(),
    }
  }
}

impl Deref for GameConfig {
  type Target = server_config::GameConfig;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

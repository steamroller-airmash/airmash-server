use std::{borrow::Cow, time::Duration};

use crate::CowString;
use protocol::MobType;

#[derive(Clone, Debug)]
pub struct MobProto {
  pub name: CowString,
  pub server_type: MobType,

  /// The default amount of time that this mob will remain in the game before it
  /// despawns.
  pub lifetime: Duration,

  /// The effect that this mob has on the player.
  pub effect: CowString,
}

impl MobProto {
  pub const fn shield() -> Self {
    Self {
      name: Cow::Borrowed("shield"),
      server_type: MobType::Shield,
      lifetime: Duration::from_secs(60),
      effect: Cow::Borrowed("shield"),
    }
  }

  pub const fn inferno() -> Self {
    Self {
      name: Cow::Borrowed("inferno"),
      server_type: MobType::Inferno,
      lifetime: Duration::from_secs(60),
      effect: Cow::Borrowed("inferno"),
    }
  }

  pub const fn upgrade() -> Self {
    Self {
      name: Cow::Borrowed("upgrade"),
      server_type: MobType::Upgrade,
      lifetime: Duration::from_secs(60),
      effect: Cow::Borrowed("upgrade"),
    }
  }
}

//! Utility accessors for components
//! that are commonly used together.

mod clock;
pub(crate) mod fire_missiles;
mod isalive;
mod send_to_all;
mod send_to_team;
mod send_to_team_visible;
mod send_to_visible;

pub use self::clock::ReadClock;
pub use self::fire_missiles::FireMissiles;
pub use self::isalive::IsAlive;
pub use self::send_to_all::SendToAll;
pub use self::send_to_team::SendToTeam;
pub use self::send_to_team_visible::SendToTeamVisible;
pub use self::send_to_visible::SendToVisible;

pub type SendToPlayer<'a> = SendToAll<'a>;

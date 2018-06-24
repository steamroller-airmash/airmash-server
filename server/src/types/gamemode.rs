
use specs::Entity;
use types::*;

use std::any::Any;

/// Base trait for a gamemode visitor.
/// It is used to allow for specialization
pub trait GameModeVisitor: Any {}

pub trait GameMode {
	fn assign_team(&mut self, player: Entity) -> Team;
	fn respawn_pos(&mut self, player: Entity, team: Team) -> Position;

}

pub trait VisitableGameMode: GameMode {
	fn visit(&mut self, visitor: &GameModeVisitor);
}


use specs::Entity;
use types::*;

use std::any::Any;

pub trait GameMode: Sync + Send {
	fn assign_team(&mut self, player: Entity) -> Team;
	fn respawn_pos(&mut self, player: Entity, team: Team) -> Position;

	fn visit(&mut self, _visitor: &Any) -> Box<Any> {
		Box::new(())
	}
}

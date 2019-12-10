
use crate::ecs::Entity;

#[derive(Error, Debug, Copy, Clone)]
#[error("The entity (id: {}, gen: {}) has already been deleted", .0.id(), .0.gen())]
pub struct EntityDead(Entity);

impl EntityDead {
	pub(super) fn new(ent: Entity) -> Self {
		Self(ent)
	}

	pub fn entity(&self) -> Entity {
		self.0
	}
}


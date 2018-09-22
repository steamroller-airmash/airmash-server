use hibitset;
use specs::*;

use component::flag::{IsDead, IsSpectating};

#[derive(SystemData)]
pub struct IsAlive<'a> {
	pub is_spec: ReadStorage<'a, IsSpectating>,
	pub is_dead: ReadStorage<'a, IsDead>,
}

impl<'a> IsAlive<'a> {
	pub fn get(&self, ent: Entity) -> bool {
		let is_spec = self.is_spec.get(ent).is_none();
		let is_dead = self.is_dead.get(ent).is_none();

		is_spec && is_dead
	}

	pub fn mask<'b: 'a>(
		&'b self,
	) -> hibitset::BitSetNot<hibitset::BitSetOr<&hibitset::BitSet, &hibitset::BitSet>> {
		!(self.is_spec.mask() | self.is_dead.mask())
	}
}

use hibitset;

use crate::component::flag::{IsDead, IsSpectating};
use crate::ecs::prelude::*;

/// SystemData utility to easily check whether a player
/// is currently alive.
#[derive(SystemData)]
pub struct IsAlive<'a> {
    is_spec: ReadStorage<'a, IsSpectating>,
    is_dead: ReadStorage<'a, IsDead>,
}

impl<'a> IsAlive<'a> {
    pub fn get(&self, ent: Entity) -> bool {
        let is_not_spec = self.is_spec.get(ent).is_none();
        let is_not_dead = self.is_dead.get(ent).is_none();

        is_not_spec && is_not_dead
    }

    pub fn mask<'b: 'a>(
        &'b self,
    ) -> hibitset::BitSetNot<hibitset::BitSetOr<&hibitset::BitSet, &hibitset::BitSet>> {
        !(self.is_spec.mask() | self.is_dead.mask())
    }
}

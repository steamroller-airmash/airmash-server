use crate::types::collision::Grid;

use std::ops::{Deref, DerefMut};

macro_rules! impl_newtype_deref_grid {
	($name:ty) => {
		impl Deref for $name {
			type Target = Grid;

			fn deref(&self) -> &Grid {
				&self.0
			}
		}

		impl DerefMut for $name {
			fn deref_mut(&mut self) -> &mut Grid {
				&mut self.0
			}
		}
	};
}

/// Precomputed grid for collision applications.
///
/// Contains the hitcircles of all planes that
/// are currently not dead or in spec.
#[derive(Debug, Default)]
pub struct PlaneGrid(pub Grid);

/// Precomputed grid containing missile locations.
#[derive(Debug, Default)]
pub struct MissileGrid(pub Grid);

/// Precomputed grid containing *only* player locations.
///
/// This is primarily targeted at visiblity operations.
/// Otherwise you want [`PlaneGrid`].
#[derive(Debug, Default)]
pub struct PlayerGrid(pub Grid);

/// Precomputed grid containing powerup locations (not hit circles).
#[derive(Debug, Default)]
pub struct PowerupGrid(pub Grid);

impl_newtype_deref_grid!(PlaneGrid);
impl_newtype_deref_grid!(MissileGrid);
impl_newtype_deref_grid!(PlayerGrid);
impl_newtype_deref_grid!(PowerupGrid);

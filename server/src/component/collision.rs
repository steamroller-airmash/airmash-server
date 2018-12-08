use types::collision::Grid;

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

use specs::*;

#[derive(Default, Clone, Debug)]
pub struct KeyState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub fire: bool,
    pub special: bool,
    // This might not be the best place to
    // keep these, can be moved later if
    // necessary
    pub stealthed: bool,
    pub flagspeed: bool,
}

impl Component for KeyState {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

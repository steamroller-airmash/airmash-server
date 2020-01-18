use airmash_protocol::*;

#[derive(Debug, Clone, Copy)]
pub struct Mob {
    pub accel: Accel,
    pub vel: Velocity,
    pub pos: Position,
    pub max_speed: Speed,

    pub owner: Option<u16>,
    pub id: u16,
    pub ty: MobType,
}

impl Mob {
    /// Whether this mob is a missile.
    pub fn missile(&self) -> bool {
        use self::MobType::*;

        match self.ty {
            PredatorMissile | GoliathMissile | MohawkMissile | TornadoSingleMissile
            | TornadoTripleMissile | ProwlerMissile => true,
            Upgrade | Shield | Inferno => false,
        }
    }
}

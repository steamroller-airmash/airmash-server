mod keystate;
mod powerup;
mod protocol;

pub mod counter;
pub mod flag;
pub mod time;

pub use self::inner::{AssociatedConnection, Name, Session};
pub use self::keystate::KeyState;
pub use self::powerup::{PowerupExt, Powerups};

pub use crate::protocol::{
    Accel, AccelScalar, Distance, Energy, EnergyRegen, FlagCode as Flag, Health, HealthRegen,
    Level, MobType as Mob, PlaneType as Plane, PlayerStatus as Status, Position, Rotation, Score,
    Speed, Team, Upgrades, Velocity,
};

mod inner {
    use crate::ecs::HashMapStorage;
    use crate::resource::socket::SocketId;
    use uuid::Uuid;

    #[derive(Clone, Debug, Component)]
    #[storage(HashMapStorage)]
    pub struct Name(pub String);

    #[derive(Clone, Debug, Component)]
    #[storage(HashMapStorage)]
    pub struct Session(pub Option<Uuid>);

    #[derive(Clone, Debug, Component)]
    pub struct AssociatedConnection(pub SocketId);
}

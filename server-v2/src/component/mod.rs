mod keystate;
mod powerup;
mod protocol;

pub mod counter;
pub mod flag;
pub mod time;

pub use self::inner::{AssociatedConnection, MissileOwner, Name, Session, Upgrades};
pub use self::keystate::KeyState;
pub use self::powerup::{PowerupExt, Powerups};

pub use crate::protocol::{
    Accel, AccelScalar, Distance, Energy, EnergyRegen, FlagCode as Flag, Health, HealthRegen,
    Level, MobType as Mob, PlaneType as Plane, PlayerStatus as Status, Position,
    PowerupType as Powerup, Rotation, Score, Speed, Team, Time, Velocity,
};

mod inner {
    use crate::ecs::{EntityRef, HashMapStorage};
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

    #[derive(Default, Clone, Copy, Debug, Component)]
    pub struct Upgrades {
        pub speed: u8,
        pub defense: u8,
        pub energy: u8,
        pub missile: u8,
        pub unused: u16,
    }

    #[derive(Clone, Debug, Component)]
    #[storage(HashMapStorage)]
    pub struct MissileOwner(pub EntityRef);
}

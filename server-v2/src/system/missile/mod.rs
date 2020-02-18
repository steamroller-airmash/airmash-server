//! Systems that deal with missiles. This includes: firing,
//! updating, etc.

mod despawn;
mod fire;
mod notify;
mod update;

pub use self::despawn::{despawn_terrain_collision, missile_despawn};
pub use self::fire::missile_fire;
pub use self::notify::missile_notify_fire;
pub use self::update::missile_update;

pub fn register(builder: &mut crate::ecs::Builder) {
    builder
        .with::<missile_fire>()
        .with::<missile_update>()
        .with::<missile_notify_fire>()
        .with::<missile_despawn>()
        .with::<despawn_terrain_collision>();
}

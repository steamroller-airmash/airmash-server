//! Systems that deal with missiles. This includes: firing,
//! updating, etc.

mod fire;
mod notify;
mod update;

pub use self::fire::missile_fire;
pub use self::notify::missile_notify;
pub use self::update::missile_update;

pub fn register(builder: &mut crate::ecs::Builder) {
    builder
        .with::<missile_fire>()
        .with::<missile_update>()
        .with::<missile_notify>();
}

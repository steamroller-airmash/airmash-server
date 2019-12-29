
mod protocol;
mod keystate;

pub mod flag;
pub mod time;
pub mod counter;

pub use self::keystate::KeyState;
pub use self::inner::{Name, Session};

mod inner {
	use crate::ecs::HashMapStorage;
	use uuid::Uuid;

	#[derive(Clone, Debug, Component)]
	#[storage(HashMapStorage)]
	pub struct Name(pub String);

	#[derive(Clone, Debug, Component)]
	#[storage(HashMapStorage)]
	pub struct Session(pub Option<Uuid>);
}

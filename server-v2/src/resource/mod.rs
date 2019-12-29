//! Resources used within the airmash server.
//!

mod connections;

pub mod builtin;
pub mod packet;
pub mod socket;
pub mod event;

pub use self::connections::Connections;
pub use self::inner::PlayerNames;

mod inner {
	use fxhash::FxHashMap;
	use crate::ecs::Entity;

	#[derive(Clone, Debug, Default)]
	pub struct PlayerNames(pub FxHashMap<String, Entity>);

}


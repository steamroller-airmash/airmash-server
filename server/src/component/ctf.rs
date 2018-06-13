use shrev::*;
use specs::*;
use types::Position;

use std::time::Instant;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsFlag;

#[derive(Copy, Clone, Debug, Default, Component)]
pub struct FlagCarrier(pub Option<Entity>);

#[derive(Copy, Clone, Debug, Default, Component)]
pub struct FlagPosition(pub Position);

#[derive(Copy, Clone, Debug)]
pub enum FlagEventType {
	PickUp,
	Return,
	Capture,
	Drop,
}

#[derive(Copy, Clone, Debug)]
pub struct FlagEvent {
	pub ty: FlagEventType,
	/// Player that carried out the action (capturer, player that returned)
	pub player: Option<Entity>,
	pub flag: Entity,
}

#[derive(Copy, Clone, Debug, Component)]
pub struct LastDrop {
	pub player: Option<Entity>,
	pub time: Instant,
}

pub type OnFlag = EventChannel<FlagEvent>;
pub type OnFlagReader = ReaderId<FlagEvent>;


use specs::*;
use shrev::*;
use types::Position;

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
	Drop
}

#[derive(Copy, Clone, Debug)]
pub struct FlagEvent {
	pub ty: FlagEventType,
	pub carrier: Option<Entity>,
	pub flag: Entity,
}

pub type OnFlag = EventChannel<FlagEvent>;
pub type OnFlagReader = ReaderId<FlagEvent>;

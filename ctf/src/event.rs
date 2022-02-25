use airmash::Entity;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug)]
pub struct GameStartEvent;

#[derive(Copy, Clone, Debug)]
pub struct GameEndEvent {
  pub winning_team: u16,
}

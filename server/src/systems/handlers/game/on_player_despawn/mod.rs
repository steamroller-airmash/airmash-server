pub type AllDespawnHandlers = ();

pub type KnownEventSources = (
  super::on_player_killed::CreateDespawnEvent,
  super::on_spectate_event::CreateDespawnEvent,
  super::on_leave::CreateDespawnEvent,
);

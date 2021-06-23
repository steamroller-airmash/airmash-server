use crate::component::channel::OnPlayerRespawn;
use crate::component::event::{PlayerRespawn, PlayerRespawnPrevStatus};
use crate::component::flag::{IsDead, IsSpectating};
use crate::task::TaskData;
use specs::Entity;

use std::time::Duration;

/// Wait 2 seconds then clear the `IsDead` flag and
/// respawn the player if they haven't requested to
/// go into spectate.
pub async fn death_cooldown(mut task: TaskData, player: Entity) {
  task.sleep_for(Duration::from_secs(2)).await;

  task.write_storage::<IsDead, _, _>(|mut storage: specs::WriteStorage<IsDead>| {
    storage.remove(player)
  });

  let is_spec = task.fetch::<IsSpectating>(player).is_some();

  if !is_spec {
    task.write_resource::<OnPlayerRespawn, _, _>(|mut channel| {
      channel.single_write(PlayerRespawn {
        player,
        prev_status: PlayerRespawnPrevStatus::Dead,
      });
    });
  }
}

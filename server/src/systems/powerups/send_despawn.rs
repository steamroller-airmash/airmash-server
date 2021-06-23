use specs::*;

use crate::types::collision::*;
use crate::types::systemdata::*;

use crate::component::event::*;
use crate::component::flag::*;
use crate::systems;
use crate::utils::*;

use crate::protocol::server::MobDespawn;
use crate::protocol::DespawnType;

#[derive(Default)]
pub struct SendDespawn;

#[derive(SystemData)]
pub struct SendDespawnData<'a> {
  entities: Entities<'a>,
  conns: SendToVisible<'a>,

  is_player: ReadStorage<'a, IsPlayer>,
}

impl EventHandlerTypeProvider for SendDespawn {
  type Event = PlayerPowerupCollision;
}

impl<'a> EventHandler<'a> for SendDespawn {
  type SystemData = SendDespawnData<'a>;

  fn on_event(&mut self, evt: &PlayerPowerupCollision, data: &mut Self::SystemData) {
    let Collision(c1, c2) = evt.0;

    let (_, upgrade) = match data.is_player.get(c1.ent) {
      Some(_) => (c1, c2),
      None => (c2, c1),
    };

    if !data.entities.is_alive(upgrade.ent) {
      return;
    }

    data.conns.send_to_visible(
      upgrade.pos,
      MobDespawn {
        id: upgrade.ent.into(),
        ty: DespawnType::Collided,
      },
    );
  }
}

system_info! {
  impl SystemInfo for SendDespawn {
    type Dependencies = systems::collision::PlayerPowerupCollisionSystem;
  }
}

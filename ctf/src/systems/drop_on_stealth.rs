use specs::*;

use crate::component::*;
use crate::server::component::event::PlayerStealth;
use crate::server::systems::handlers::game::on_player_despawn::KnownEventSources;
use crate::server::utils::{EventHandler, EventHandlerTypeProvider};
use crate::server::*;

#[derive(Default)]
pub struct DropOnStealth;

#[derive(SystemData)]
pub struct DropOnStealthData<'a> {
  channel: Write<'a, OnFlag>,
  entities: Entities<'a>,
  carrier: WriteStorage<'a, FlagCarrier>,
  isflag: ReadStorage<'a, IsFlag>,
}

impl EventHandlerTypeProvider for DropOnStealth {
  type Event = PlayerStealth;
}

impl<'a> System<'a> for DropOnStealth {
  type SystemData = DropOnStealthData<'a>;

  fn run(&mut self, _data: Self::SystemData) {}
}

impl<'a> EventHandler<'a> for DropOnStealth {
  type SystemData = DropOnStealthData<'a>;

  fn on_event(&mut self, evt: &PlayerStealth, data: &mut Self::SystemData) {
    let player = evt.player;
    let channel = &mut data.channel;

    (&*data.entities, &mut data.carrier, &data.isflag)
      .join()
      .filter(|(_, carrier, _)| carrier.0.is_some())
      .filter(|(_, carrier, _)| carrier.0.unwrap() == player)
      .for_each(|(ent, carrier, _)| {
        channel.single_write(FlagEvent {
          ty: FlagEventType::Drop,
          player: Some(player),
          flag: ent,
        });

        carrier.0 = None;
      });
  }
}

system_info! {
  impl SystemInfo for DropOnStealth {
    type Dependencies = (
      // FIXME: I don't think this is necessary, need to investigate more
      KnownEventSources,
      super::PickupFlag,
    );
  }
}

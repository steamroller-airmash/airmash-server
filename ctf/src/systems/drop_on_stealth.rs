use specs::*;

use component::*;
use server::component::event::PlayerStealth;
use server::systems::handlers::game::on_despawn::KnownEventSources;
use server::utils::event_handler::{EventHandler, EventHandlerTypeProvider};
use server::*;

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

impl SystemInfo for DropOnStealth {
	type Dependencies = (KnownEventSources, super::PickupFlagSystem);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

use specs::*;

use component::event::TimerEvent;
use component::flag::IsDead;
use consts::timer::CLEAR_DEAD_FLAG;
use utils::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

#[derive(Default)]
pub struct ClearDeadFlag;

#[derive(SystemData)]
pub struct ClearDeadFlagData<'a> {
	entities: Entities<'a>,
	is_dead: WriteStorage<'a, IsDead>,
}

impl EventHandlerTypeProvider for ClearDeadFlag {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for ClearDeadFlag {
	type SystemData = ClearDeadFlagData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *CLEAR_DEAD_FLAG {
			return;
		}

		let player = match evt.data {
			Some(ref dat) => match (*dat).downcast_ref::<Entity>() {
				Some(val) => *val,
				None => {
					error!("Unable to downcast TimerEvent data to Entity! Event will be skipped.");
					return;
				}
			},
			None => return,
		};

		if !data.entities.is_alive(player) {
			return;
		}

		data.is_dead.remove(player);
	}
}

impl SystemInfo for ClearDeadFlag {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}

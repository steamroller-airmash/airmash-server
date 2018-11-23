use specs::*;

use component::event::TimerEvent;
use consts::timer::DELETE_ENTITY;

use utils::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

#[derive(Default)]
pub struct DeleteEntity;

#[derive(SystemData)]
pub struct DeleteEntityData<'a> {
	entities: Entities<'a>,
}

impl EventHandlerTypeProvider for DeleteEntity {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for DeleteEntity {
	type SystemData = DeleteEntityData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *DELETE_ENTITY {
			return;
		}

		let ent = match evt.data {
			Some(ref data) => match (*data).downcast_ref::<Entity>() {
				Some(val) => *val,
				None => {
					error!("Unable to downcast TimerEvent data to Entity!");
					return;
				}
			},
			None => return,
		};

		if !data.entities.is_alive(ent) {
			return;
		}

		data.entities.delete(ent).unwrap();
	}
}

impl SystemInfo for DeleteEntity {
	type Dependencies = ();

	fn new() -> Self {
		Self::default()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}

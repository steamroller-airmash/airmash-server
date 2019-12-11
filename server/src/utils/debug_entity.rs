use specs::prelude::*;

use std::fmt::{Debug, Formatter, Result};

#[derive(SystemDataCustom)]
pub struct DebugAdapter<'a> {
	lazy: Read<'a, LazyUpdate>,
}

impl<'a> DebugAdapter<'a> {
	pub fn lazy_debug(&self, ent: Entity) {
		self.lazy.exec_mut(move |world| {
			info!("{:#?}", DebugPrinter { ent, world });
		})
	}
}
struct DebugPrinter<'a> {
	ent: Entity,
	world: &'a World,
}

impl<'a> Debug for DebugPrinter<'a> {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
		write!(fmt, "<cannot debug entity>")
		// self.world.debug_entity(self.ent).fmt(fmt)
	}
}

use shred::SystemData;
use shrev::*;
use specs::*;

use std::any::Any;

use dispatch::{SystemInfo, SystemParent};
use utils::maybe_init::MaybeInit;

pub trait EventHandler<'a> {
	type SystemData: SystemData<'a>;
	type Event: 'static;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);
	}

	fn on_event(&mut self, evt: &Self::Event);
}

pub struct EventHandlerWrapper<'a, T>
where
	T: EventHandler<'a>,
	T::Event: Sync + Send,
{
	reader: MaybeInit<ReaderId<T::Event>>,
	handler: T,
}

impl<'a, T> System<'a> for EventHandlerWrapper<'a, T>
where
	T: EventHandler<'a>,
	T::Event: Sync + Send,
{
	type SystemData = (Read<'a, EventChannel<T::Event>>, T::SystemData);

	fn setup(&mut self, res: &mut Resources) {
		Read::<EventChannel<T::Event>>::setup(res);
		T::setup(&mut self.handler, res);

		self.reader = MaybeInit::new(res.fetch_mut::<EventChannel<T::Event>>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.0.read(&mut self.reader) {
			self.handler.on_event(evt);
		}
	}
}

impl<'a, T> SystemInfo for EventHandlerWrapper<'a, T>
where
	T: SystemInfo + EventHandler<'a>,
	T::Event: Send + Sync + 'static,
{
	type Dependencies = T::Dependencies;

	fn name() -> &'static str {
		T::name()
	}

	fn new() -> Self {
		unimplemented!()
	}

	fn new_args(args: Box<Any>) -> Self {
		Self {
			reader: MaybeInit::uninit(),
			handler: T::new_args(args),
		}
	}
}

impl<'a, T> SystemParent for EventHandlerWrapper<'a, T>
where
	T: for<'c> System<'c> + EventHandler<'a> + Send + SystemInfo,
	T::Event: Send + Sync + 'static,
{
	type Inner = T;
}

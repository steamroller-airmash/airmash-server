use shred::SystemData;
use shrev::*;
use specs::*;

use std::any::Any;

use dispatch::SystemInfo;
use utils::maybe_init::MaybeInit;

pub trait EventHandlerTypeProvider {
	type Event: Send + Sync + 'static;
}

pub trait EventHandler<'a>: EventHandlerTypeProvider + Send {
	type SystemData: SystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);
	}

	fn on_event(&mut self, evt: &Self::Event, data: &mut Self::SystemData);
}

pub(crate) struct EventHandlerWrapper<T>
where
	T: EventHandlerTypeProvider,
{
	reader: MaybeInit<ReaderId<T::Event>>,
	handler: T,
}

impl<'a, T> System<'a> for EventHandlerWrapper<T>
where
	T: EventHandler<'a> + EventHandlerTypeProvider,
{
	type SystemData = (
		Read<'a, EventChannel<T::Event>>,
		<T as EventHandler<'a>>::SystemData,
	);

	fn setup(&mut self, res: &mut Resources) {
		Read::<EventChannel<T::Event>>::setup(res);
		T::setup(&mut self.handler, res);

		self.reader = MaybeInit::new(res.fetch_mut::<EventChannel<T::Event>>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.0.read(&mut self.reader) {
			self.handler.on_event(evt, &mut data.1);
		}
	}
}

impl<T> SystemInfo for EventHandlerWrapper<T>
where
	T: SystemInfo + EventHandlerTypeProvider,
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

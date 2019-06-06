use shred::SystemData;
use shrev::*;
use specs::*;

use std::any::Any;

use dispatch::SystemInfo;
use utils::MaybeInit;

/// Supplies the type of event that for which this
/// system is listening. Used along with [`EventHandler`].
pub trait EventHandlerTypeProvider {
	/// The type of event that the event handler is
	/// listening for.
	type Event: Send + Sync + 'static;
}

/// Trait for a system which handles a single event.
///
/// Once registered, systems that implement this will
/// have `on_event` called once for every time that
/// the desired event (as specified in
/// [`EventHandlerTypeProvider`]).
///
/// # Notes
/// The default capacity of an [`EventChannel`](shrev::EventChannel)
/// is 50. If more elements are added to the channel in a single
/// frame than it's capacity, some events will get dropped before
/// any event handlers get a chance to read them. If this is happening
/// then initializing the problematic channels with a greater capacity
/// will allow more events to occur per frame.
pub trait EventHandler<'a>: EventHandlerTypeProvider + Send {
	/// All resources that this system uses (reads or writes).
	type SystemData: SystemData<'a>;

	/// Setup any system resources.
	///
	/// If you override this, remember to call
	/// `Self::SystemData::setup`, otherwise some
	/// of the resources that this system depends
	/// upon may not be initialized.
	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);
	}

	/// Handle an event. All processing for the event
	/// should happen here.
	fn on_event(&mut self, evt: &Self::Event, data: &mut Self::SystemData);
}

/// This is the type that actually implements
/// the `System` trait for all `EventHandlers`.
///
/// There should be no need to ever have to
/// use this directly (except within `dispatch`).
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

	fn new_args(args: Box<dyn Any>) -> Self {
		Self {
			reader: MaybeInit::uninit(),
			handler: T::new_args(args),
		}
	}
}

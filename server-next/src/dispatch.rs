use crate::AirmashWorld;
use anymap::AnyMap;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use linkme::distributed_slice;

#[distributed_slice]
pub static HANDLERS: [fn(&EventDispatcher)] = [..];

/// Marker trait for events.
///
/// This just verifies that the event is `Send + Sync + 'static` and should
/// never have to be implemented manually.
pub trait Event: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Event for T {}

/// Trait for an event handler.
pub trait EventHandler<E: Event>: 'static {
  fn on_event(&mut self, event: &E, world: &mut AirmashWorld);
}

impl<F, E> EventHandler<E> for F
where
  F: FnMut(&E, &mut AirmashWorld) + 'static,
  E: Event
{
  fn on_event(&mut self, event: &E, world: &mut AirmashWorld) {
    self(event, world);
  }
}

type HandlerList<E> = Vec<Box<dyn EventHandler<E>>>;

trait DelayedEvent {
  fn dispatch(&mut self, world: &mut AirmashWorld, map: &mut AnyMap);
}

struct ConcreteDelayedEvent<E>(Option<E>);

impl<E: Event> DelayedEvent for ConcreteDelayedEvent<E> {
  fn dispatch(&mut self, world: &mut AirmashWorld, map: &mut AnyMap) {
    BaseEventDispatcher::dispatch_raw(self.0.take().unwrap(), world, map);
  }
}

/// Event dispatcher.
///
/// This is responsible for maintaining all the event handlers and invoking them
/// whenever an event is dispatched.
struct BaseEventDispatcher {
  lists: RefCell<AnyMap>,
  queue: RefCell<VecDeque<Box<dyn DelayedEvent>>>,
}

impl BaseEventDispatcher {
  pub fn new() -> Self {
    Self {
      lists: RefCell::new(AnyMap::new()),
      queue: RefCell::new(VecDeque::new()),
    }
  }

  fn register<E, H>(&self, handler: H)
  where
    H: EventHandler<E>,
    E: Event,
  {
    let mut lists = self.lists.borrow_mut();
    lists
      .entry::<HandlerList<E>>()
      .or_insert_with(Vec::new)
      .push(Box::new(handler));
  }

  fn dispatch_raw<E>(event: E, world: &mut AirmashWorld, lists: &mut AnyMap)
  where
    E: Event,
  {
    if let Some(list) = lists.get_mut::<HandlerList<E>>() {
      for handler in list {
        handler.on_event(&event, world);
      }
    }
  }

  fn dispatch<E>(&self, event: E, world: &mut AirmashWorld)
  where
    E: Event,
  {
    let mut lists = match self.lists.try_borrow_mut() {
      Ok(lists) => lists,
      Err(_) => {
        let mut queue = self.queue.borrow_mut();
        queue.push_back(Box::new(ConcreteDelayedEvent(Some(event))));
        return;
      }
    };

    Self::dispatch_raw(event, world, &mut lists);

    while let Some(mut event) = self.next_queued() {
      event.dispatch(world, &mut lists);
    }
  }

  fn next_queued(&self) -> Option<Box<dyn DelayedEvent>> {
    self.queue.borrow_mut().pop_front()
  }
}

#[derive(Clone)]
pub struct EventDispatcher {
  dispatcher: Rc<BaseEventDispatcher>,
}

impl EventDispatcher {
  pub fn new() -> Self {
    Self {
      dispatcher: Rc::new(BaseEventDispatcher::new()),
    }
  }

  pub fn register<E, H>(&self, handler: H)
  where
    H: EventHandler<E>,
    E: Event,
  {
    self.dispatcher.register(handler)
  }

  pub fn dispatch<E>(&self, event: E, world: &mut AirmashWorld)
  where
    E: Event,
  {
    self.dispatcher.dispatch(event, world);
  }
}

impl Default for EventDispatcher {
  fn default() -> Self {
    Self::new()
  }
}

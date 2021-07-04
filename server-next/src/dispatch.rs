use crate::AirmashWorld;
use anymap::AnyMap;
use linkme::distributed_slice;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[distributed_slice]
pub static HANDLERS: [fn(&EventDispatcher)] = [..];

pub const DEFAULT_PRIORITY: i32 = 0;

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
  E: Event,
{
  fn on_event(&mut self, event: &E, world: &mut AirmashWorld) {
    self(event, world);
  }
}

struct HandlerWithPriority<E>(i32, Box<dyn EventHandler<E>>);
type HandlerList<E> = Vec<HandlerWithPriority<E>>;

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
  /// Cleanup tasks that need to be done after all the derivative events have
  /// been executed.
  ///
  /// This is not exposed outside of this crate.
  cleanup: RefCell<VecDeque<Box<dyn FnMut(&mut AirmashWorld)>>>,
}

impl BaseEventDispatcher {
  pub fn new() -> Self {
    Self {
      lists: RefCell::new(AnyMap::new()),
      queue: RefCell::new(VecDeque::new()),
      cleanup: RefCell::new(VecDeque::new()),
    }
  }

  fn register_with_priority<E, H>(&self, priority: i32, handler: H)
  where
    H: EventHandler<E>,
    E: Event,
  {
    let mut lists = self.lists.borrow_mut();
    let list = lists.entry::<HandlerList<E>>().or_insert_with(Vec::new);

    list.push(HandlerWithPriority(priority, Box::new(handler)));
    list.sort();
  }

  fn dispatch_raw<E>(event: E, world: &mut AirmashWorld, lists: &mut AnyMap)
  where
    E: Event,
  {
    if let Some(list) = lists.get_mut::<HandlerList<E>>() {
      for handler in list.iter_mut() {
        handler.1.on_event(&event, world);
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

    drop(lists);
    let mut cleanup = self.cleanup.borrow_mut();
    for mut func in cleanup.drain(..) {
      func(world);
    }
  }

  /// Add a function to the cleanup queue. If no event is currently executing
  /// then it will be executed immediately.
  fn add_cleanup<F>(&self, world: &mut AirmashWorld, func: F)
  where
    F: FnOnce(&mut AirmashWorld) + 'static,
  {
    if self.lists.try_borrow_mut().is_ok() {
      func(world);
      return;
    }

    let mut cleanup = self.cleanup.borrow_mut();
    let mut func = Some(func);
    cleanup.push_back(Box::new(move |game| {
      (func.take().unwrap())(game);
    }));
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

  pub fn register_with_priority<E, H>(&self, priority: i32, handler: H)
  where
    H: EventHandler<E>,
    E: Event,
  {
    self.dispatcher.register_with_priority(priority, handler)
  }

  pub fn dispatch<E>(&self, event: E, world: &mut AirmashWorld)
  where
    E: Event,
  {
    self.dispatcher.dispatch(event, world);
  }

  pub(crate) fn cleanup<F>(&self, world: &mut AirmashWorld, func: F)
  where
    F: FnOnce(&mut AirmashWorld) + 'static,
  {
    self.dispatcher.add_cleanup(world, func);
  }
}

impl Default for EventDispatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl<E> PartialEq for HandlerWithPriority<E> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<E> PartialOrd for HandlerWithPriority<E> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(other.0.cmp(&self.0))
  }
}

impl<E> Ord for HandlerWithPriority<E> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.0.cmp(&self.0)
  }
}

impl<E> Eq for HandlerWithPriority<E> {}

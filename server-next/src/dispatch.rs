use crate::AirmashGame;
use anymap::AnyMap;
use linkme::distributed_slice;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[distributed_slice]
pub static AIRMASH_EVENT_HANDLERS: [fn(&EventDispatcher)] = [..];

pub const DEFAULT_PRIORITY: i32 = 0;

/// Marker trait for events.
///
/// This just verifies that the event is `Send + Sync + 'static` and should
/// never have to be implemented manually.
pub trait Event: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Event for T {}

/// Trait for an event handler.
pub trait EventHandler<E: Event>: 'static {
  fn on_event(&mut self, event: &E, game: &mut AirmashGame);
}

impl<F, E> EventHandler<E> for F
where
  F: FnMut(&E, &mut AirmashGame) + 'static,
  E: Event,
{
  fn on_event(&mut self, event: &E, world: &mut AirmashGame) {
    self(event, world);
  }
}

struct HandlerWithPriority<E>(i32, Box<dyn EventHandler<E>>);
type HandlerList<E> = Vec<HandlerWithPriority<E>>;

trait DelayedEvent {
  fn dispatch(&mut self, world: &mut AirmashGame, map: &mut AnyMap);
}

struct ConcreteDelayedEvent<E>(Option<E>);

impl<E: Event> DelayedEvent for ConcreteDelayedEvent<E> {
  fn dispatch(&mut self, world: &mut AirmashGame, map: &mut AnyMap) {
    BaseEventDispatcher::dispatch_raw(self.0.take().unwrap(), world, map);
  }
}

/// Event dispatcher.
///
/// This is responsible for maintaining all the event handlers and invoking them
/// whenever an event is dispatched.
#[allow(clippy::type_complexity)]
struct BaseEventDispatcher {
  lists: RefCell<AnyMap>,
  queue: RefCell<VecDeque<Box<dyn DelayedEvent>>>,
  /// Cleanup tasks that need to be done after all the derivative events have
  /// been executed.
  ///
  /// This is not exposed outside of this crate.
  cleanup: RefCell<VecDeque<Box<dyn FnMut(&mut AirmashGame)>>>,
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

  fn dispatch_raw<E>(event: E, world: &mut AirmashGame, lists: &mut AnyMap)
  where
    E: Event,
  {
    if let Some(list) = lists.get_mut::<HandlerList<E>>() {
      for handler in list.iter_mut() {
        handler.1.on_event(&event, world);
      }
    }
  }

  fn dispatch<E>(&self, event: E, world: &mut AirmashGame)
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
  fn add_cleanup<F>(&self, world: &mut AirmashGame, func: F)
  where
    F: FnOnce(&mut AirmashGame) + 'static,
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

/// Raw handle to the event dispatcher.
///
/// Most uses of this type should instead go through the methods on
/// [`AirmashGame`] instead of interacting directly with the event dispatcher.
/// This type is reference counted so cloning it will give back another handle
/// to the same underlying event dispatcher.
#[derive(Clone)]
pub struct EventDispatcher {
  dispatcher: Rc<BaseEventDispatcher>,
}

impl EventDispatcher {
  /// Create a new event dispatcher with no registered event handlers.
  pub fn new() -> Self {
    Self {
      dispatcher: Rc::new(BaseEventDispatcher::new()),
    }
  }

  /// Register a new event handler with the provided priority.
  pub fn register_with_priority<E, H>(&self, priority: i32, handler: H)
  where
    H: EventHandler<E>,
    E: Event,
  {
    self.dispatcher.register_with_priority(priority, handler)
  }

  /// Dispatch the provided event and execute all the resulting event handlers
  /// in decreasing order of priority.
  pub fn dispatch<E>(&self, event: E, world: &mut AirmashGame)
  where
    E: Event,
  {
    self.dispatcher.dispatch(event, world);
  }

  /// Schedule a cleanup task to be run after the current event and all events
  /// triggered as part of running the current event have completed. If no event
  /// is current then the function is executed immediately.
  ///
  /// There is no prioritization available here, cleanup functions are executed
  /// in the order that they are registered at the end of the top-level
  /// [`dispatch`] call that is currently executing.
  ///
  /// [`dispatch`]: crate::EventDispatcher::dispatch
  pub fn cleanup<F>(&self, world: &mut AirmashGame, func: F)
  where
    F: FnOnce(&mut AirmashGame) + 'static,
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

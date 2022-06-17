use std::cell::{Cell, RefCell, UnsafeCell};
use std::collections::BinaryHeap;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use futures_task::{ArcWake, Context, Poll};
use slab::Slab;

use crate::AirmashGame;

#[derive(Clone, Copy)]
struct TaskContext {
  game: *mut AirmashGame,
  timeout: *mut Option<Instant>,
}

thread_local! {
  static TASK_CONTEXT: Cell<Option<TaskContext>> = Cell::new(None);
}

pub struct GameRef(PhantomData<*mut ()>);

impl GameRef {
  fn new() -> Self {
    Self(PhantomData)
  }

  pub async fn sleep_until(&mut self, until: Instant) {
    TimeoutFuture::new(until, self).await
  }

  pub async fn sleep_for(&mut self, time: Duration) {
    let this_frame = self.this_frame();
    self.sleep_until(this_frame + time).await
  }
}

impl Deref for GameRef {
  type Target = AirmashGame;

  fn deref(&self) -> &Self::Target {
    let game = TASK_CONTEXT.with(|ctx| ctx.get().map(|c| c.game));
    unsafe { &*game.expect("Attempted to dereference a GameRef outside of a game context") }
  }
}

impl DerefMut for GameRef {
  fn deref_mut(&mut self) -> &mut Self::Target {
    let game = TASK_CONTEXT.with(|ctx| ctx.get().map(|c| c.game));
    unsafe { &mut *game.expect("Attempted to dereference a GameRef outside of a game context") }
  }
}

/// Task scheduler handle.
///
/// This can be used to either
/// 1. schedule a function to run at some time in the future, or
/// 2. schedule an async function to run a series of work in the future
#[derive(Clone)]
pub struct TaskScheduler {
  inner: Rc<RefCell<TaskSchedulerImpl>>,
  spawn: Sender<Box<dyn Future<Output = ()>>>,
}

impl TaskScheduler {
  pub fn new() -> Self {
    let (tx, rx) = crossbeam_channel::unbounded();

    Self {
      inner: Rc::new(RefCell::new(TaskSchedulerImpl::new(rx))),
      spawn: tx,
    }
  }

  /// Schedule an async function. By using the async methods on [`GameRef`] you
  /// can perform multiple waits across frames easily.
  ///
  /// # Safety
  /// By using this function you are guaranteeing that you will never keep a
  /// mutable reference to [`AirmashGame`] (gotten by dereferencing [`GameRef`])
  /// across an await.
  ///
  /// If you are only using the async methods on [`GameRef`] to suspend then
  /// this is impossible, however external futures don't have that limitation.
  pub unsafe fn spawn<Fut, Fn>(&self, func: Fn)
  where
    Fn: FnOnce(GameRef) -> Fut + 'static,
    Fut: Future<Output = ()> + 'static,
  {
    let _ = self.spawn.send(Box::new(async {
      func(GameRef::new()).await;
    }));
  }

  /// Schedule a function to run the first frame after `time`.
  pub fn schedule<T>(&self, time: Instant, task: T)
  where
    T: FnOnce(&mut AirmashGame) + 'static,
  {
    unsafe {
      self.spawn(move |mut game: GameRef| async move {
        game.sleep_until(time).await;
        task(&mut *game)
      });
    }
  }

  pub(crate) fn update(&self, game: &mut AirmashGame) {
    let inner = Rc::clone(&self.inner);
    let mut inner = inner.borrow_mut();
    inner.turn(game);
  }
}

impl Default for TaskScheduler {
  fn default() -> Self {
    Self::new()
  }
}

struct TaskItem {
  task: Pin<Box<dyn Future<Output = ()>>>,
  waker: Arc<TaskWaker>,
  last: u64,
}

struct TaskSchedulerImpl {
  tasks: Slab<TaskItem>,
  queue: BinaryHeap<TimeoutDesc>,
  external: Receiver<usize>,
  sender: Sender<usize>,
  incoming: Receiver<Box<dyn Future<Output = ()>>>,

  turn: u64,
}

impl TaskSchedulerImpl {
  fn new(incoming: Receiver<Box<dyn Future<Output = ()>>>) -> Self {
    let (tx, rx) = crossbeam_channel::unbounded();

    Self {
      tasks: Slab::default(),
      queue: BinaryHeap::new(),
      external: rx,
      sender: tx,
      incoming,
      turn: 0,
    }
  }

  pub fn turn(&mut self, game: &mut AirmashGame) {
    let timeout = UnsafeCell::new(None);
    self.turn += 1;

    let this_frame = game.this_frame();

    TASK_CONTEXT.with(|ctx| {
      ctx.set(Some(TaskContext {
        game,
        timeout: timeout.get(),
      }));
    });
    let _guard = DropGuard::new(|| {
      TASK_CONTEXT.with(|ctx| ctx.set(None));
    });

    while let Ok(task) = self.incoming.try_recv() {
      let entry = self.tasks.vacant_entry();
      let id = entry.key();

      entry.insert(TaskItem {
        // SAFETY: This is just Box::into_pin which is still unstable.
        task: unsafe { Pin::new_unchecked(task) },
        waker: Arc::new(TaskWaker {
          id,
          channel: self.sender.clone(),
        }),
        last: 0,
      });

      self.poll_task(id, &timeout);
    }

    while let Some(desc) = self.queue.peek() {
      if desc.time > this_frame {
        break;
      }

      let desc = self.queue.pop().unwrap();
      self.poll_task(desc.task, &timeout);
    }

    while let Ok(taskid) = self.external.try_recv() {
      self.poll_task(taskid, &timeout);
    }
  }

  fn poll_task(&mut self, taskid: usize, timeout: &UnsafeCell<Option<Instant>>) {
    let mut task = &mut self.tasks[taskid];

    if task.last == self.turn {
      return;
    }

    task.last = self.turn;
    let waker = futures_task::waker_ref(&task.waker);
    let mut context = Context::from_waker(&waker);

    match Future::poll(task.task.as_mut(), &mut context) {
      Poll::Ready(()) => {
        self.tasks.remove(taskid);
        // SAFETY: Timeout is only accessed otherwise from within Future::poll
        //         and references do not outlive it.
        unsafe { *timeout.get() = None };
      }
      _ => {
        // SAFETY: Timeout is only accessed otherwise from within Future::poll
        //         and references do not outlive it.
        if let Some(timeout) = unsafe { (*timeout.get()).take() } {
          self.queue.push(TimeoutDesc {
            task: taskid,
            time: timeout,
          });
        }
      }
    };
  }
}

struct TaskWaker {
  id: usize,
  channel: Sender<usize>,
}

impl ArcWake for TaskWaker {
  fn wake_by_ref(arc_self: &Arc<Self>) {
    let _ = arc_self.channel.send(arc_self.id);
  }
}

struct DropGuard<F: FnMut()> {
  func: F,
}

impl<F: FnMut()> DropGuard<F> {
  pub fn new(func: F) -> Self {
    Self { func }
  }
}

impl<F: FnMut()> Drop for DropGuard<F> {
  fn drop(&mut self) {
    (self.func)();
  }
}

struct TimeoutDesc {
  time: Instant,
  task: usize,
}

impl PartialEq for TimeoutDesc {
  fn eq(&self, other: &Self) -> bool {
    self.time.eq(&other.time)
  }
}

impl PartialOrd for TimeoutDesc {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    other.time.partial_cmp(&self.time)
  }
}

impl Ord for TimeoutDesc {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.time.cmp(&self.time)
  }
}

impl Eq for TimeoutDesc {}

struct TimeoutFuture<'g> {
  game: &'g mut GameRef,
  timeout: Instant,
}

impl<'g> TimeoutFuture<'g> {
  fn new(timeout: Instant, game: &'g mut GameRef) -> Self {
    Self { timeout, game }
  }
}

impl<'g> Future for TimeoutFuture<'g> {
  type Output = ();

  fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
    let this_frame = self.game.this_frame();

    if this_frame >= self.timeout {
      Poll::Ready(())
    } else {
      TASK_CONTEXT.with(|ctx| {
        let ctx = ctx.get().unwrap();

        // SAFETY: The scheduler does not hold a reference over poll and the reference
        //         never escapes this function.
        let mref = unsafe { &mut *ctx.timeout };
        *mref = Some(mref.unwrap_or(self.timeout).min(self.timeout));
      });

      Poll::Pending
    }
  }
}

use crate::AirmashGame;
use std::cell::UnsafeCell;
use std::rc::Rc;
use std::{collections::BinaryHeap, time::Instant};

/// Marker trait for a task.
///
/// This is implemented for any type which implements [`FnOnce`] with the
/// correct signature so it usually will not need to be implemented manually
/// unless you want to use a custom task type.
pub trait Task {
  fn invoke(self, game: &mut AirmashGame);
}

/// Handler to the task scheduler.
///
/// This can be used to schedule a function to run at some time in the future.
/// As soon as that time has passed the requested function will be executed.
/// This is useful for scheduling events that happen every so often. However, if
/// you need to perform timed tasks for a whole class of entities then you are
/// probably better off running some work every frame by adding a handler for
/// the [`Frame`](crate::event::Frame) event.
///
/// Note that this type is just a handle to the underlying task scheduler so
/// cloning it will still refer to the same underlying instance.
#[derive(Clone, Default)]
pub struct TaskScheduler {
  detail: Rc<TaskSchedulerDetail>,
}

impl TaskScheduler {
  /// Create a new `TaskScheduler` with no tasks.
  pub fn new() -> Self {
    Self::default()
  }

  /// Schedule a task to run at `time`.
  pub fn schedule<T>(&self, time: Instant, task: T)
  where
    T: Task + 'static,
  {
    self.detail.schedule(time, task);
  }

  pub(crate) fn update(&self, now: Instant, game: &mut AirmashGame) {
    self.detail.update(now, game);
  }
}

struct TaskDesc {
  time: Instant,
  task: Box<dyn FnMut(&mut AirmashGame)>,
}

#[derive(Default)]
struct TaskSchedulerDetail {
  tasks: UnsafeCell<BinaryHeap<TaskDesc>>,
}

impl TaskSchedulerDetail {
  fn schedule<T>(&self, time: Instant, task: T)
  where
    T: Task + 'static,
  {
    // SAFETY: This is safe since this method is not reentrant and will never be
    //         called on multiple threads due to TaskScheduler not being Send.
    let tasks = unsafe { &mut *self.tasks.get() };

    let mut task = Some(task);
    tasks.push(TaskDesc {
      time,
      task: Box::new(move |game: &mut AirmashGame| {
        let task = task.take().unwrap();
        task.invoke(game);
      }),
    });
  }

  fn update(&self, now: Instant, game: &mut AirmashGame) {
    let ptr = self.tasks.get();

    // SAFETY: This safe since this method will never be called from multiple
    //         threads and the call to desc.task happens when we do not have a
    //         reference to the value within the UnsafeCell.
    while let Some(desc) = unsafe { (*ptr).peek() } {
      if desc.time > now {
        break;
      }
      drop(desc);

      // SAFETY: See note above.
      let mut desc = unsafe { (*ptr).pop().unwrap() };
      (desc.task)(game);
    }
  }
}

impl PartialEq for TaskDesc {
  fn eq(&self, other: &Self) -> bool {
    self.time.eq(&other.time)
  }
}

impl PartialOrd for TaskDesc {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    other.time.partial_cmp(&self.time)
  }
}

impl Ord for TaskDesc {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.time.cmp(&self.time)
  }
}

impl Eq for TaskDesc {}

impl<F> Task for F
where
  F: FnOnce(&mut AirmashGame),
{
  fn invoke(self, game: &mut AirmashGame) {
    self(game);
  }
}

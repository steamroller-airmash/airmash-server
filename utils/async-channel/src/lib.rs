//! Efficient async MPSC channel. This crate is a thin wrapper around
//! crossbeam-channel that only provides a few primitives needed by the airmash
//! server.

use arc_swap::ArcSwapOption;
use crossbeam_channel as c;
use std::{
  future::Future,
  pin::Pin,
  sync::Arc,
  task::{Context, Poll, Waker},
};

pub use c::{RecvError, SendError, TryRecvError};

struct Shared {
  waker: ArcSwapOption<Waker>,
}

impl Shared {
  pub fn new() -> Self {
    Self {
      waker: ArcSwapOption::new(None),
    }
  }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
  let shared = Arc::new(Shared::new());
  let (tx, rx) = c::unbounded();

  let tx = Sender {
    tx,
    shared: shared.clone(),
  };
  let rx = Receiver { rx, shared };

  (tx, rx)
}

pub struct Sender<T> {
  tx: c::Sender<T>,
  shared: Arc<Shared>,
}

impl<T> Sender<T> {
  pub fn send(&mut self, msg: T) -> Result<(), SendError<T>> {
    self.tx.send(msg)?;

    if let Some(ref waker) = *self.shared.waker.load() {
      waker.wake_by_ref();
    }

    Ok(())
  }
}

pub struct Receiver<T> {
  rx: c::Receiver<T>,
  shared: Arc<Shared>,
}

impl<T> Receiver<T> {
  pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
    self.rx.try_recv()
  }

  pub async fn recv(&mut self) -> Result<T, RecvError> {
    RecvFuture::new(self).await
  }

  pub fn is_empty(&self) -> bool {
    self.rx.is_empty()
  }

  pub fn len(&self) -> usize {
    self.rx.len()
  }
}

struct RecvFuture<'a, T> {
  recv: &'a mut Receiver<T>,
}

impl<'a, T> RecvFuture<'a, T> {
  fn new(recv: &'a mut Receiver<T>) -> Self {
    Self { recv }
  }
}

impl<T> Unpin for RecvFuture<'_, T> {}

impl<'a, T> Future for RecvFuture<'a, T> {
  type Output = Result<T, RecvError>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    match self.recv.rx.try_recv() {
      Ok(v) => Poll::Ready(Ok(v)),
      Err(TryRecvError::Disconnected) => Poll::Ready(Err(RecvError)),
      Err(TryRecvError::Empty) => {
        let waker = Arc::new(cx.waker().clone());
        self.recv.shared.waker.store(Some(waker));

        Poll::Pending
      }
    }
  }
}

impl<'a, T> Drop for RecvFuture<'a, T> {
  fn drop(&mut self) {
    self.recv.shared.waker.store(None);
  }
}

//! Fast, zero-allocation*, bounded queue implementation.
//!
//! * Except on queue creation.

use std::mem;

#[derive(Clone, Debug)]
pub struct BoundedQueue<T> {
  front: usize,
  back: usize,
  vals: Box<[Option<T>]>,
}

impl<T> BoundedQueue<T> {
  /// Return the current length of the queue.
  ///
  /// The length of the queue can never increase
  /// beyond the [`capacity()`] of the queue.
  ///
  /// # Example
  /// Inserting elements when the queue is full
  /// does not change the length.
  /// ```
  /// # extern crate bounded_queue;
  /// # use bounded_queue::*;
  /// # fn main() {
  /// let mut queue = BoundedQueue::new(1);
  ///
  /// // The queue is empty
  /// assert!(queue.len() == 0);
  ///
  /// queue.push(1u32);
  /// // Now the queue has a length of 1
  /// assert!(queue.len() == 1);
  ///
  /// queue.push(2u32);
  /// // The queue still has a length of 2
  /// assert!(queue.len() == 1);
  /// # }
  /// ```
  pub fn len(&self) -> usize {
    self.front - self.back
  }
  /// Check if the queue is empty
  pub fn is_empty(&self) -> bool {
    self.front == self.back
  }
  /// Check to see if there is any
  /// room left in the queue.
  ///
  /// This being true implies that inserting any
  /// more elements into the queue will cause elements
  /// to be dropped out of the queue.
  ///
  /// # Example
  /// Here an element is inserted into a full
  /// queue and the queue remains full.
  /// ```
  /// # extern crate bounded_queue;
  /// # use bounded_queue::*;
  /// # fn main() {
  /// let mut queue = BoundedQueue::new(1);
  ///
  /// // The queue isn't full yet
  /// assert!(!queue.is_full());
  /// queue.push(1u32);
  ///
  /// // Now the queue is full
  /// assert!(queue.is_full());
  ///
  /// queue.push(2);
  /// // The queue is still full
  /// assert!(queue.is_full());
  /// # }
  /// ```
  pub fn is_full(&self) -> bool {
    self.len() == self.capacity()
  }
  /// The capacity of the underlying buffer used
  /// by the queue.
  ///
  /// This value is fixed at queue construction time.
  pub fn capacity(&self) -> usize {
    self.vals.len()
  }

  fn last(&self) -> usize {
    self.back % self.capacity()
  }
  fn first(&self) -> usize {
    self.front % self.capacity()
  }

  /// Insert a new element into the back of the queue. If the
  /// queue is full, then it returns the element
  /// that was removed.
  ///
  /// # Example
  /// When the queue is empty push returns none, but
  /// when the queue is full push returns the element
  /// that was bumped out.
  /// ```
  /// # extern crate bounded_queue;
  /// # use bounded_queue::*;
  /// # fn main() {
  /// let mut queue = BoundedQueue::new(1);
  ///
  /// assert!(queue.push(1) == None);
  /// assert!(queue.push(2) == Some(1));
  /// # }
  /// ```
  pub fn push(&mut self, val: T) -> Option<T> {
    let val = mem::replace(&mut self.vals[self.first()], Some(val));

    if self.is_full() {
      self.back += 1;
    }
    self.front += 1;

    val
  }

  /// Remove an element from the front of the queue.
  /// Returns `None` when empty.
  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      return None;
    }

    let last = self.last();
    self.back += 1;

    mem::replace(&mut self.vals[last], None)
  }
  /// Get a reference to the first element of the queue,
  /// without modifying the queue. Returns `None` if
  /// empty.
  pub fn peek(&self) -> Option<&T> {
    if self.is_empty() {
      return None;
    }

    Some(self.vals[self.last()].as_ref().unwrap())
  }
}

impl<T: Clone> BoundedQueue<T> {
  /// Creates a new bounded queue with a fixed size.
  ///
  /// # Example
  /// Create a new queue.
  /// ```
  /// # extern crate bounded_queue;
  /// # use bounded_queue::*;
  /// # fn main() {
  /// // Create a new queue
  /// let mut queue = BoundedQueue::new(5);
  ///
  /// // use it here
  /// # assert!(queue.is_empty());
  /// # queue.push(0);
  /// # }
  /// ```
  ///
  /// # Panics
  /// This function will panic if `size == 0`.
  pub fn new(size: usize) -> Self {
    if size == 0 {
      panic!("A bounded queue may not be constructed with a size of 0!");
    }

    Self {
      front: 0,
      back: 0,
      vals: vec![None; size].into_boxed_slice(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::BoundedQueue;

  #[test]
  #[should_panic]
  pub fn cannot_create_zero_sized_queue() {
    let _ = BoundedQueue::<u8>::new(0);
  }

  #[test]
  pub fn empty_queue_is_empty() {
    let x = BoundedQueue::<u8>::new(5);

    assert!(x.is_empty());
  }

  #[test]
  pub fn push_on_full_returns_some() {
    let mut x = BoundedQueue::<u8>::new(1);

    assert!(x.push(0).is_none());
    assert!(x.push(1).is_some());
  }

  #[test]
  pub fn peek_on_empty_returns_none() {
    let x = BoundedQueue::<u8>::new(1);
    assert!(x.peek().is_none());
  }

  #[test]
  pub fn pop_on_empty_returns_none() {
    let mut x = BoundedQueue::<u8>::new(1);
    assert!(x.pop().is_none());
  }
}

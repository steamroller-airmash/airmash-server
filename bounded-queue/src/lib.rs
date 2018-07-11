//! Fast, zero-allocation, bounded queue implementation.

use std::mem;

#[derive(Clone, Debug)]
pub struct BoundedQueue<T> {
    front: usize,
    back: usize,
    vals: Box<[Option<T>]>,
}

impl<T> BoundedQueue<T> {
    pub fn len(&self) -> usize {
        self.front - self.back
    }
    pub fn is_empty(&self) -> bool {
        self.front == self.back
    }
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }
    pub fn capacity(&self) -> usize {
        self.vals.len()
    }

    fn last(&self) -> usize {
        self.back % self.capacity()
    }
    fn first(&self) -> usize {
        self.front % self.capacity()
    }

    pub fn push(&mut self, val: T) -> Option<T> {
        let val = mem::replace(&mut self.vals[self.first()], Some(val));

        if self.is_full() {
            self.back += 1;
        }
        self.front += 1;

        val
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let last = self.last();
        self.back += 1;

        mem::replace(&mut self.vals[last], None)
    }
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(self.vals[self.last()].as_ref().unwrap())
    }
}

impl<T: Clone> BoundedQueue<T> {
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

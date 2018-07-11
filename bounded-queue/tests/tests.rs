#![feature(test)]

extern crate bounded_queue;
extern crate test;

use bounded_queue::BoundedQueue;

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

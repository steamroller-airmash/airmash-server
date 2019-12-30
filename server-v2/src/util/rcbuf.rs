use std::fmt;
use std::mem::ManuallyDrop;
use std::ops::{Deref, Index};
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Reference counted buffer.
pub struct RcBuf<T> {
    counter: NonNull<AtomicUsize>,
    data: ManuallyDrop<Vec<T>>,
}

unsafe impl<T: Send + Sync> Send for RcBuf<T> {}
unsafe impl<T: Send + Sync> Sync for RcBuf<T> {}

impl<T> RcBuf<T> {
    pub fn new(data: Vec<T>) -> Self {
        let counter = Box::new(AtomicUsize::new(1));

        Self {
            counter: unsafe { NonNull::new_unchecked(Box::into_raw(counter)) },
            data: ManuallyDrop::new(data),
        }
    }

    pub fn clone(buf: &Self) -> Self {
        <Self as Clone>::clone(buf)
    }

    fn vec(&self) -> &Vec<T> {
        unsafe { &*(&self.data as *const _ as *const Vec<T>) }
    }

    pub fn as_slice(&self) -> &[T] {
        self.vec().as_slice()
    }

    pub fn strong_count(&self) -> usize {
        unsafe { self.counter.as_ref().load(Ordering::Relaxed) }
    }

    pub fn into_inner(mut self) -> Result<Vec<T>, Self> {
        unsafe {
            let count = self.counter.as_ref().fetch_sub(1, Ordering::Relaxed);

            if count == 1 {
                // Since count==0, the drop impl won't drop the vector.
                Ok(manuallydrop_take(&mut self.data))
            } else {
                self.counter.as_ref().fetch_add(1, Ordering::Relaxed);
                Err(self)
            }
        }
    }
}

impl<T> Index<usize> for RcBuf<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.vec()[index]
    }
}

impl<T> Deref for RcBuf<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &*self.vec()
    }
}

impl<T> Clone for RcBuf<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.counter.as_ref().fetch_add(1, Ordering::Relaxed);

            Self {
                counter: self.counter,
                // Copy the data without actually copying it.
                // The drop implementation will make sure that
                // it only gets dropped once.
                data: ptr::read(&self.data),
            }
        }
    }
}

unsafe fn manuallydrop_take<T>(val: &mut ManuallyDrop<T>) -> T {
    ManuallyDrop::into_inner(ptr::read(val))
}

impl<T> Drop for RcBuf<T> {
    fn drop(&mut self) {
        unsafe {
            let count = self.counter.as_ref().fetch_sub(1, Ordering::Relaxed);

            if count == 1 {
                let _ = Box::from_raw(self.counter.as_ptr());
                ManuallyDrop::drop(&mut self.data);
            }
        };
    }
}

impl<T: fmt::Debug> fmt::Debug for RcBuf<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(fmt)
    }
}

impl<T> From<Vec<T>> for RcBuf<T> {
    fn from(data: Vec<T>) -> RcBuf<T> {
        RcBuf::new(data)
    }
}

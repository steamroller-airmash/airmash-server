use super::VTable;

use std::alloc::{alloc, dealloc, Layout};
use std::marker::Unsize;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone)]
struct VecMeta<V: VTable> {
    vtable: V,
    drop: unsafe fn(*mut ()),
    offset: usize,
}

pub struct AnyVec<V: VTable> {
    data: AlignUnsafeVec,
    // VTables + offsets
    meta: Vec<VecMeta<V>>,
}

impl<V: VTable> AnyVec<V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<T>(&mut self, val: T)
    where
        T: Unsize<V::Trait>,
    {
        let vtable = V::from_existing(&val);
        let offset = self.data.push(val);

        self.meta.push(VecMeta {
            vtable,
            offset,
            drop: drop_in_place::<T>,
        });
    }

    pub fn iter_mut(&mut self) -> AnyVecMutIterator<V> {
        AnyVecMutIterator::new(self)
    }
    pub fn iter(&self) -> AnyVecIterator<V> {
        AnyVecIterator::new(self)
    }
}

impl<V: VTable> Drop for AnyVec<V> {
    fn drop(&mut self) {
        for meta in self.meta.drain(..) {
            let ptr = self.data.mut_ptr_at(meta.offset);

            unsafe {
                (meta.drop)(ptr as *mut ());
            }
        }
    }
}

pub struct AnyVecIterator<'a, V: VTable> {
    vec: &'a AnyVec<V>,
    index: usize,
}

impl<'a, V: VTable> AnyVecIterator<'a, V> {
    pub fn new(vec: &'a AnyVec<V>) -> Self {
        Self { vec, index: 0 }
    }
}

impl<'a, V: VTable> Iterator for AnyVecIterator<'a, V> {
    type Item = &'a V::Trait;

    fn next(&mut self) -> Option<Self::Item> {
        let meta = self.vec.meta.get(self.index)?;
        let vtable = meta.vtable.clone();
        let offset = meta.offset;

        let offset = self.vec.data.ptr_at(offset);

        // Safe since the rest of anyvec ensures that this
        // pointer is a valid pointer to the correct type
        // of object here.
        let obj = unsafe { vtable.rebuild(&*offset) };

        self.index += 1;

        Some(obj)
    }
}

pub struct AnyVecMutIterator<'a, V: VTable> {
    vec: &'a mut AnyVec<V>,
    index: usize,
}

impl<'a, V: VTable> AnyVecMutIterator<'a, V> {
    pub fn new(vec: &'a mut AnyVec<V>) -> Self {
        Self { vec, index: 0 }
    }
}

impl<'a, V: VTable> Iterator for AnyVecMutIterator<'a, V> {
    type Item = &'a mut V::Trait;

    fn next(&mut self) -> Option<Self::Item> {
        let meta = self.vec.meta.get(self.index)?;
        let vtable = meta.vtable.clone();
        let offset = meta.offset;

        let offset = self.vec.data.mut_ptr_at(offset);

        // Safe since the rest of anyvec ensures that this
        // pointer is a valid pointer to the correct type
        // of object here.
        let obj = unsafe { vtable.rebuild_mut(&mut *offset) };

        self.index += 1;

        Some(obj)
    }
}

fn align(x: usize, align: usize) -> usize {
    let rem = x % align;

    match rem {
        0 => x,
        _ => x + (align - rem),
    }
}

unsafe fn drop_in_place<T>(ptr: *mut ()) {
    assert!(!ptr.is_null());

    std::ptr::drop_in_place(ptr as *mut T);
}

impl<V: VTable> Default for AnyVec<V> {
    fn default() -> Self {
        Self {
            data: AlignUnsafeVec::new(),
            meta: Vec::new(),
        }
    }
}

struct AlignUnsafeVec {
    data: *mut MaybeUninit<u8>,
    length: usize,
    capacity: usize,
    align: usize,
}

impl AlignUnsafeVec {
    fn new() -> Self {
        Self {
            data: std::ptr::null_mut(),
            length: 0,
            capacity: 0,
            align: 16,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn align(&self) -> usize {
        self.align
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn ptr_at(&self, offset: usize) -> *const MaybeUninit<u8> {
        // Note: offset == self.len() makes sense for zero-sized types
        assert!(offset <= self.len());

        if self.data.is_null() {
            // This should only happen when this vector only contains ZSTs
            self.align() as *const _
        } else {
            self.data.wrapping_add(offset)
        }
    }
    pub fn mut_ptr_at(&mut self, offset: usize) -> *mut MaybeUninit<u8> {
        // Note: offset == self.len() makes sense for zero-sized types
        assert!(offset <= self.len());

        if self.data.is_null() {
            // This should only happen when this vector only contains ZSTs
            self.align() as *mut _
        } else {
            self.data.wrapping_add(offset)
        }
    }

    fn realloc(&mut self, align: usize, cap: usize) {
        let layout = match Layout::from_size_align(cap, align) {
            Ok(layout) => layout,
            Err(e) => unreachable!(
                "Tried to create a bad layout using align {} and size {}: {}",
                align, cap, e
            ),
        };

        if self.data.is_null() {
            self.data = unsafe { alloc(layout) as *mut MaybeUninit<u8> };

            if self.data.is_null() {
                // Everything is broken now, use the stdlib OOM handling
                std::alloc::handle_alloc_error(layout);
            }

            self.capacity = cap;
            self.align = align;
        } else {
            // Note: We can't take advantage of realloc in the general
            //       case here since we might be allocating with a
            //       more stringent alignment.
            // TODO: Use realloc when the alignment is not changed.

            // Put any and all panics before we could change anything
            // to avoid leaking memory if possible.
            let existing = match Layout::from_size_align(self.capacity, self.align) {
                Ok(existing) => existing,
                Err(e) => unreachable!(
                    "AlignUnsafeVec has a bad layout with align {} and capacity {}: {}",
                    self.align, self.capacity, e
                ),
            };

            // Safe since layout is definitely valid here (checked by panic above)
            let new_ptr = unsafe { alloc(layout) as *mut MaybeUninit<u8> };
            if new_ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }

            // Safe since both pointers are in a valid state here
            // - we've checked new_ptr for validity
            // - self.data must be valid since otherwise we'd be
            //   in the other if branch here.
            unsafe {
                // Update new buffer with existing data
                std::ptr::copy_nonoverlapping(self.data, new_ptr, self.len());
            }

            let old_data = std::mem::replace(&mut self.data, new_ptr);
            self.capacity = cap;
            self.align = align;

            // Safe since self.data is valid (see comment above)
            unsafe {
                dealloc(old_data as *mut _, existing);
            }
        }
    }

    fn ensure_can_push(&mut self, layout: Layout) {
        // Zero-size types don't force us to allocate but might
        // affect alignment. If they need a greater alignment
        // then we'll need to reallocate to support that.
        if layout.size() == 0 && self.data.is_null() {
            self.align = self.align().max(layout.align());
            return;
        }

        let mut should_realloc = self.data.is_null();
        let mut new_cap = self.capacity();
        let mut new_align = self.align();

        let new_len = match layout.size() {
            0 => self.len(),
            _ => align(self.len(), layout.align()) + layout.size(),
        };

        if self.align < layout.align() {
            should_realloc = true;
            new_align = layout.align();
        }

        // If we don't do this the loop after will spin forever
        if new_cap == 0 {
            new_cap = 1;
        }

        while new_cap < new_len {
            should_realloc = true;
            new_cap *= 2;
        }

        if should_realloc {
            self.realloc(new_align, new_cap);
        }
    }

    pub fn push<T>(&mut self, val: T) -> usize {
        let layout = Layout::for_value(&val);
        self.ensure_can_push(layout);

        let (offset, new_len) = match layout.size() {
            // ZST pointers need only be aligned, the most efficient
            // way to do this is to have them always be at offset 0
            // since this avoids unneeded gaps in the backing representation.
            0 => (0, self.len()),
            _ => {
                let offset = align(self.len(), layout.align());
                (offset, offset + layout.size())
            }
        };

        // Safe since this offset is a valid location for a
        // value of type T. This is guaranteed by ensure_can_push.
        unsafe {
            std::ptr::write(self.mut_ptr_at(offset) as *mut T, val);
        }

        self.length = new_len;

        offset
    }
}

impl Drop for AlignUnsafeVec {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let layout = Layout::from_size_align(self.capacity, self.align)
                .expect("Failed to create layout");

            // Safe since
            // - The layout is valid (checked by panic above)
            // - Any non-null data pointer must be valid (AlignUnsafeVec invariant)
            unsafe { dealloc(self.data as *mut _, layout) }
        }
    }
}

impl Deref for AlignUnsafeVec {
    type Target = [MaybeUninit<u8>];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.data, self.length) }
    }
}

impl DerefMut for AlignUnsafeVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.length) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::vtable::AnyVTable;

    use std::rc::Rc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[repr(align(128))]
    struct Aligned;

    #[repr(align(1))]
    #[derive(Default)]
    struct Len5([u8; 5]);

    #[repr(align(1))]
    #[derive(Default)]
    struct Len3([u8; 3]);

    struct DeathCtr(Rc<AtomicUsize>);

    impl Drop for DeathCtr {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn push_zero_sized() {
        let mut vec = AlignUnsafeVec::new();
        vec.push(());

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn push_aligned_zst() {
        let mut vec = AlignUnsafeVec::new();
        vec.push(Aligned);

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.align(), 128);
    }

    #[test]
    fn zst_doesnt_make_gaps() {
        let mut vec = AlignUnsafeVec::new();

        vec.push(Len5::default());
        vec.push(Aligned);
        vec.push(Len3::default());

        assert_eq!(vec.len(), 8);
        assert_eq!(vec.align(), 128);
        assert!(vec.capacity() >= vec.len());
    }

    #[test]
    fn anyvec_drops_elements() {
        let mut vec: AnyVec<AnyVTable> = AnyVec::new();
        let ctr = Rc::new(AtomicUsize::new(0));

        vec.push(DeathCtr(Rc::clone(&ctr)));

        drop(vec);

        assert_eq!(ctr.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn anyvec_iter_correct_num_elements() {
        let mut vec: AnyVec<AnyVTable> = AnyVec::new();

        for _ in 0..13 {
            vec.push(());
        }

        let count = vec.iter_mut().take(100).count();
        assert_eq!(count, 13);
    }
}

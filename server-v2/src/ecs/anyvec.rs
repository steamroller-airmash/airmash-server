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

        Some(obj)
    }
}

fn align(x: usize, align: usize) -> usize {
    let rem = x % align;

    if rem == 0 {
        return x;
    } else {
        x + (align - rem)
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

    pub fn mut_ptr_at(&mut self, offset: usize) -> *mut MaybeUninit<u8> {
        assert!(!self.data.is_null());
        assert!(offset < self.len());

        self.data.wrapping_add(offset)
    }

    fn realloc(&mut self, align: usize, cap: usize) {
        let layout = Layout::from_size_align(align, cap);

        let layout = match layout {
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
        // Zero-size types don't ever need reallocation
        if layout.size() == 0 {
            return;
        }

        let mut should_realloc = self.data.is_null();
        let mut new_cap = self.capacity();
        let mut new_align = self.align();

        let start_offset = align(self.len(), layout.align());
        let new_len = start_offset + layout.size();

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

        let offset = align(self.len(), layout.align());
        let new_len = offset + layout.size();

        // Safe since this offset is a valid location for a
        // value of type T. This is guaranteed by ensure_can_push.
        unsafe {
            std::ptr::write(self.mut_ptr_at(offset) as *mut T, val);
        }

        self.length = new_len;

        offset
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

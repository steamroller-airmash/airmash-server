use std::any::TypeId;
use std::collections::{hash_map, HashMap};
use std::hash::{BuildHasherDefault, Hasher};
use std::marker::PhantomData;
use std::mem;

#[derive(Default)]
struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
  #[inline]
  fn write(&mut self, bytes: &[u8]) {
    self.write_u64(u64::from_ne_bytes(
      bytes.try_into().expect("input bytes had an incorrect size"),
    ));
  }

  #[inline]
  fn write_u64(&mut self, i: u64) {
    self.0 = i;
  }

  #[inline]
  fn finish(&self) -> u64 {
    self.0
  }
}

trait Any: std::any::Any {}

impl<T: std::any::Any> Any for T {}

impl dyn Any {
  pub fn is<T: Any>(&self) -> bool {
    self.type_id() == TypeId::of::<T>()
  }

  pub unsafe fn downcast_ref_unchecked<T: Any>(&self) -> &T {
    debug_assert!(self.is::<T>());
    &*(self as *const dyn Any as *const T)
  }

  pub unsafe fn downcast_mut_unchecked<T: Any>(&mut self) -> &mut T {
    debug_assert!(self.is::<T>());
    &mut *(self as *mut dyn Any as *mut T)
  }

  pub unsafe fn downcast_unchecked<T: Any>(self: Box<Self>) -> Box<T> {
    debug_assert!(self.is::<T>());

    Box::from_raw(Box::into_raw(self) as *mut T)
  }
}

pub struct AnyMap {
  map: HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<TypeIdHasher>>,
}

impl AnyMap {
  /// Create an empty map.
  #[inline]
  pub fn new() -> Self {
    Self {
      map: HashMap::with_hasher(BuildHasherDefault::default()),
    }
  }

  /// Create an empty map with the given initial capacity.
  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      map: HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
    }
  }

  /// Reserves capacity for at least `additional` more elements to be inserted
  /// in the `AnyMap`. The `AnyMap` may reserve more space to avoid frequent
  /// reallocations.
  ///
  /// # Panics
  /// Panics if the new allocation size overflows `usize`.
  #[inline]
  pub fn reserve(&mut self, additional: usize) {
    self.map.reserve(additional);
  }

  /// Shrinks the capacity of the collection as much as possible. It will drop
  /// down as much as possible while maintaining the internal rules and possibly
  /// leaving some space in accordance with the resize policy.
  #[inline]
  pub fn shrink_to_fit(&mut self) {
    self.map.shrink_to_fit();
  }

  /// Returns the number of items in the collection.
  #[inline]
  pub fn len(&self) -> usize {
    self.map.len()
  }

  /// Returns true if there are no items in the collection.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  pub fn get<T: 'static>(&self) -> Option<&T> {
    self
      .map
      .get(&TypeId::of::<T>())
      .map(|v| unsafe { v.downcast_ref_unchecked() })
  }

  pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
    self
      .map
      .get_mut(&TypeId::of::<T>())
      .map(|v| unsafe { v.downcast_mut_unchecked() })
  }

  pub fn insert<T: 'static>(&mut self, value: T) -> Option<T> {
    match self.map.entry(TypeId::of::<T>()) {
      hash_map::Entry::Occupied(mut entry) => Some(mem::replace(
        unsafe { entry.get_mut().downcast_mut_unchecked() },
        value,
      )),
      hash_map::Entry::Vacant(entry) => {
        entry.insert(Box::new(value));
        None
      }
    }
  }

  pub fn remove<T: 'static>(&mut self) -> Option<T> {
    self
      .map
      .remove(&TypeId::of::<T>())
      .map(|v| unsafe { *v.downcast_unchecked() })
  }

  pub fn contains<T: 'static>(&mut self) -> bool {
    self.map.contains_key(&TypeId::of::<T>())
  }

  pub fn entry<T: 'static>(&mut self) -> Entry<'_, T> {
    Entry::new(self.map.entry(TypeId::of::<T>()))
  }
}

pub struct OccupiedEntry<'a, V> {
  entry: hash_map::OccupiedEntry<'a, TypeId, Box<dyn Any>>,
  _type: PhantomData<V>,
}

pub struct VacantEntry<'a, V> {
  entry: hash_map::VacantEntry<'a, TypeId, Box<dyn Any>>,
  _type: PhantomData<V>,
}

pub enum Entry<'a, V> {
  Occupied(OccupiedEntry<'a, V>),
  Vacant(VacantEntry<'a, V>),
}

impl<'a, V: 'static> Entry<'a, V> {
  fn new(entry: hash_map::Entry<'a, TypeId, Box<dyn Any>>) -> Self {
    match entry {
      hash_map::Entry::Occupied(entry) => Self::Occupied(OccupiedEntry {
        entry,
        _type: PhantomData,
      }),
      hash_map::Entry::Vacant(entry) => Self::Vacant(VacantEntry {
        entry,
        _type: PhantomData,
      }),
    }
  }

  #[inline]
  pub fn or_insert(self, default: V) -> &'a mut V {
    match self {
      Entry::Occupied(entry) => entry.into_mut(),
      Entry::Vacant(entry) => entry.insert(default),
    }
  }

  #[inline]
  pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
    match self {
      Entry::Occupied(entry) => entry.into_mut(),
      Entry::Vacant(entry) => entry.insert(default()),
    }
  }

  pub fn or_default(self) -> &'a mut V
  where
    V: Default,
  {
    self.or_insert_with(Default::default)
  }

  pub fn and_modify<F: FnOnce(&mut V)>(mut self, f: F) -> Self {
    if let Entry::Occupied(entry) = &mut self {
      f(entry.get_mut())
    }

    self
  }
}

impl<'a, V: 'static> OccupiedEntry<'a, V> {
  #[inline]
  pub fn get(&self) -> &V {
    unsafe { self.entry.get().downcast_ref_unchecked() }
  }

  #[inline]
  pub fn get_mut(&mut self) -> &mut V {
    unsafe { self.entry.get_mut().downcast_mut_unchecked() }
  }

  #[inline]
  pub fn into_mut(self) -> &'a mut V {
    unsafe { self.entry.into_mut().downcast_mut_unchecked() }
  }

  #[inline]
  pub fn insert(&mut self, value: V) -> V {
    mem::replace(
      unsafe { self.entry.get_mut().downcast_mut_unchecked() },
      value,
    )
  }

  #[inline]
  pub fn remove(self) -> V {
    unsafe { *self.entry.remove().downcast_unchecked() }
  }
}

impl<'a, V: 'static> VacantEntry<'a, V> {
  #[inline]
  pub fn insert(self, value: V) -> &'a mut V {
    unsafe { self.entry.insert(Box::new(value)).downcast_mut_unchecked() }
  }
}

#[cfg(test)]
mod tests {
  use std::hash::Hash;

  use super::*;

  #[test]
  fn verify_typeid_hash() {
    fn verify(typeid: TypeId) {
      let mut hasher = TypeIdHasher::default();
      typeid.hash(&mut hasher);

      assert_eq!(hasher.finish(), unsafe { core::mem::transmute(typeid) })
    }

    verify(TypeId::of::<usize>());
    verify(TypeId::of::<()>());
    verify(TypeId::of::<str>());
    verify(TypeId::of::<dyn core::fmt::Debug>());
    verify(TypeId::of::<Vec<u8>>());
  }
}

use super::{DynStorage, DynSystem};

pub trait VTable: Sized + Clone {
    type Trait: ?Sized + 'static;

    /// Extract the VTable pointer for from the trait object
    /// and use it to create a vtable instance.
    fn from_existing(val: &Self::Trait) -> Self;

    /// Reconstruct a trait object from an existing data pointer.
    ///
    /// # Safety
    /// This is UB unless this vtable was constructed from
    /// an object of the same type as that pointed to by
    /// the data pointer.
    ///
    /// It is also UB if the resulting reference outlives the
    /// provided data pointer.
    unsafe fn rebuild<'a, T>(&self, data: &'a T) -> &'a Self::Trait
    where
        T: ?Sized;

    /// Reconstruct a mutable trait object from an existing data
    /// pointer.
    ///
    /// # Safety
    /// This is UB unless this vtable was constructed from
    /// an object of the same type as that pointed to by
    /// the data pointer.
    unsafe fn rebuild_mut<'a, T>(&self, data: &'a mut T) -> &'a mut Self::Trait
    where
        T: ?Sized;
}

macro_rules! declare_vtable {
	{
		$(
			$( #[$attr:meta] )*
			$vis:vis struct $vtable:ident : $trait:path;
		)*
	} => {
		$(
			$( #[$attr] )*
			#[derive(Copy, Clone)]
			$vis struct $vtable(*mut ());

			impl crate::ecs::VTable for $vtable {
				type Trait = dyn $trait;

				fn from_existing(val: &dyn $trait) -> Self {
					use ::std::{raw::TraitObject, mem};

					let raw: TraitObject = unsafe { mem::transmute(val) };
					Self(raw.vtable)
				}

				unsafe fn rebuild<'a, T>(&self, data: &'a T) -> &'a Self::Trait
				where
					T: ?Sized
				{
					use ::std::{raw::TraitObject, mem};

					let raw = TraitObject {
						data: data as *const T as *mut (),
						vtable: self.0
					};

					mem::transmute(raw)
				}

				unsafe fn rebuild_mut<'a, T>(&self, data: &'a mut T) -> &'a mut Self::Trait
				where
					T: ?Sized
				{
					use ::std::{raw::TraitObject, mem};

					let raw = TraitObject {
						data: data as *mut T as *mut (),
						vtable: self.0
					};

					mem::transmute(raw)
				}
			}

			unsafe impl Send for $vtable {}
			unsafe impl Sync for $vtable {}
		)*
	}
}

declare_vtable! {
    pub struct DynStorageVTable: DynStorage;
    pub(super) struct DynSystemVTable: DynSystem;
}

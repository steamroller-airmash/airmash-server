use super::{DynStorage, Storage};
use super::VecStorage;
use hibitset::{BitSet, BitSetLike};

macro_rules! decl_custom {
	{
		$(
			$( #[$attr:meta] )*
			$vis:vis struct $name:ident : $parent:ty => $inner:ty ;
		)*
	} => {$(
		$( #[$attr] )*
		#[derive(Default)]
		$vis struct $name($parent);

		impl $name {
			pub fn new() -> Self {
				Self::default()
			}
		}

		impl DynStorage for $name {
			fn mask(&self) -> &BitSet {
				<Self as Storage<$inner>>::mask(self)
			}

			fn remove(&mut self, ent: u32) {
				<Self as Storage<$inner>>::remove(self, ent);
			}
		
			fn remove_all(&mut self, mask: &BitSet) {
				<Self as Storage<$inner>>::remove_all(self, mask);
			}
		}

		impl Storage<$inner> for $name {
			fn mask(&self) -> &BitSet {
				<$parent as Storage<_>>::mask(&self.0)
			}

			fn insert(&mut self, ent: u32, val: $inner) -> Option<$inner> {
				self.0.insert(ent, val)
			}
			fn remove(&mut self, ent: u32) -> Option<$inner> {
				<$parent as Storage<_>>::remove(&mut self.0, ent)
			}

			fn remove_all<B: BitSetLike>(&mut self, bits: B) {
				<$parent as Storage<_>>::remove_all(&mut self.0, bits)
			}

			fn get(&self, ent: u32) -> Option<&$inner> {
				self.0.get(ent)
			}
			fn get_mut(&mut self, ent: u32) -> Option<&mut $inner> {
				self.0.get_mut(ent)
			}

			unsafe fn get_unchecked(&self, ent: u32) -> &$inner {
				self.0.get_unchecked(ent)
			}
			unsafe fn get_mut_unchecked(&mut self, ent: u32) -> &mut $inner {
				self.0.get_mut_unchecked(ent)
			}
		}
	)*}
}

decl_custom!{
}

use std::ops::{Index, IndexMut};

use fnv::FnvHashMap;

#[derive(Clone, Debug)]
pub struct Array2D<T> {
	elems: FnvHashMap<(usize, usize), T>,
	dims: (usize, usize),
}

impl<T: Default + Clone> Array2D<T> {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			dims: (width, height),
			elems: FnvHashMap::default(),
		}
	}
}

impl<T> Array2D<T> {
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.elems.values()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.elems.values_mut()
	}

	pub fn size(&self) -> (usize, usize) {
		self.dims
	}
}

impl<T> Index<(usize, usize)> for Array2D<T> {
	type Output = T;

	fn index(&self, idx: (usize, usize)) -> &Self::Output {
		&self.elems[&idx]
	}
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> {
	fn index_mut<'a>(&'a mut self, idx: (usize, usize)) -> &'a mut Self::Output {
		self.elems.get_mut(&idx).unwrap()
	}
}

use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Array2D<T> {
	elems: Vec<T>,
	dims: (usize, usize),
}

impl<T: Default + Clone> Array2D<T> {
	pub fn new(width: usize, height: usize) -> Self {
		let mut elems = vec![];
		elems.resize(width * height, T::default());

		Self { dims: (width, height), elems }
	}
}

impl<T> Array2D<T> {
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.elems.iter()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.elems.iter_mut()
	}

	pub fn size(&self) -> (usize, usize) {
		self.dims
	}
}

impl<T> IntoIterator for Array2D<T> {
	type Item = T;
	type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.elems.into_iter()
	}
}

impl<T> Index<(usize, usize)> for Array2D<T> {
	type Output = T;

	fn index(&self, idx: (usize, usize)) -> &Self::Output {
		&self.elems[idx.1 * self.dims.0 + idx.0]
	}
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> {
	fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
		&mut self.elems[idx.1 * self.dims.0 + idx.0]
	}
}

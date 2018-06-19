use fnv::FnvHashMap;

#[derive(Clone, Debug)]
pub struct Array2D<T> {
	elems: FnvHashMap<(usize, usize), T>,
	dims: (usize, usize),
}

impl<T> Array2D<T> {
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

	pub fn get(&self, idx: (usize, usize)) -> Option<&T> {
		self.elems.get(&idx)
	}
}

impl<T: Default> Array2D<T> {
	pub fn get_or_insert(&mut self, idx: (usize, usize)) -> &mut T {
		if self.elems.contains_key(&idx) {
			return self.elems.get_mut(&idx).unwrap();
		}

		assert!(
			idx.0 < self.dims.0 && idx.1 < self.dims.1,
			"{}: {:?} >= {:?} || {:?} >= {:?}",
			"Out of bounds index in Array2D",
			idx.0,
			self.dims.0,
			idx.1,
			self.dims.1
		);

		self.elems.insert(idx, T::default());
		self.elems.get_mut(&idx).unwrap()
	}
}
